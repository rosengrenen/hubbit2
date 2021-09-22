use async_graphql::{guard::Guard, Context, InputObject, Object};
use log::error;
use uuid::Uuid;

use crate::{
  models::GammaUser,
  repositories::{
    device::{CreateDevice, DeviceRepository, UpdateDevice},
    session::SessionRepository,
  },
};

use super::{AuthGuard, HubbitSchemaError, HubbitSchemaResult};

pub struct Device {
  pub id: Uuid,
}

#[Object]
impl Device {
  async fn id(&self) -> Uuid {
    self.id
  }

  async fn address(&self, context: &Context<'_>) -> HubbitSchemaResult<String> {
    let device_repo = context.data_unchecked::<DeviceRepository>();
    let device = device_repo.get_by_id(self.id).await.map_err(|e| {
      error!("[Schema error] {:?}", e);
      HubbitSchemaError::InternalError
    })?;
    Ok(device.address)
  }

  async fn name(&self, context: &Context<'_>) -> HubbitSchemaResult<String> {
    let device_repo = context.data_unchecked::<DeviceRepository>();
    let device = device_repo.get_by_id(self.id).await.map_err(|e| {
      error!("[Schema error] {:?}", e);
      HubbitSchemaError::InternalError
    })?;
    Ok(device.name)
  }

  async fn is_active(&self, context: &Context<'_>) -> HubbitSchemaResult<bool> {
    let device_repo = context.data_unchecked::<DeviceRepository>();
    let device = device_repo.get_by_id(self.id).await.map_err(|e| {
      error!("[Schema error] {:?}", e);
      HubbitSchemaError::InternalError
    })?;
    let session_repo = context.data_unchecked::<SessionRepository>();
    let is_active = session_repo
      .is_device_active(device.address)
      .await
      .map_err(|e| {
        error!("[Schema error] {:?}", e);
        HubbitSchemaError::InternalError
      })?;
    Ok(is_active)
  }
}

#[derive(Default)]
pub struct DeviceMutation;

#[Object]
impl DeviceMutation {
  #[graphql(guard(AuthGuard()))]
  pub async fn set_devices(
    &self,
    context: &Context<'_>,
    mut data: SetDevicesInput,
  ) -> HubbitSchemaResult<Vec<Device>> {
    for device in data.devices.iter_mut() {
      device.address = device.address.to_uppercase();
      validate_mac_addr(&device.address)?;
    }

    let device_repo = context.data_unchecked::<DeviceRepository>();
    let auth_user = context.data_unchecked::<GammaUser>();
    let current_devices = device_repo.get_for_user(auth_user.id).await.map_err(|e| {
      error!("[Schema error] {:?}", e);
      HubbitSchemaError::InternalError
    })?;

    let mut devices_to_create = Vec::new();
    let mut devices_to_update = Vec::new();
    let mut devices_to_remove = Vec::new();
    for device in data.devices.iter().cloned() {
      // If device doesn't exist, add to create list, else add to update list
      if !current_devices.iter().any(|d| d.address == device.address) {
        devices_to_create.push(device);
      } else {
        devices_to_update.push(device);
      }
    }

    for device in current_devices.iter().cloned() {
      // If device doesn't exist in list of new devices, add to remove list
      if !data.devices.iter().any(|d| d.address == device.address) {
        devices_to_remove.push(device);
      }
    }

    for device in devices_to_remove {
      device_repo.delete(&device.address).await.map_err(|e| {
        error!("[Schema error] {:?}", e);
        HubbitSchemaError::InternalError
      })?;
    }

    for device in devices_to_update {
      let address = device.address.clone();
      device_repo
        .update(
          &address,
          UpdateDevice {
            address: device.address,
            name: device.name,
          },
        )
        .await
        .map_err(|e| {
          error!("[Schema error] {:?}", e);
          HubbitSchemaError::InternalError
        })?;
    }

    for device in devices_to_create {
      device_repo
        .create(CreateDevice {
          address: device.address,
          name: device.name,
          user_id: auth_user.id,
        })
        .await
        .map_err(|e| {
          error!("[Schema error] {:?}", e);
          HubbitSchemaError::InternalError
        })?;
    }

    let current_devices = device_repo.get_for_user(auth_user.id).await.map_err(|e| {
      error!("[Schema error] {:?}", e);
      HubbitSchemaError::InternalError
    })?;

    Ok(
      current_devices
        .iter()
        .map(|d| Device { id: d.id })
        .collect(),
    )
  }
}

#[derive(InputObject)]
pub struct SetDevicesInput {
  devices: Vec<DeviceInput>,
}

#[derive(Clone, InputObject)]
pub struct DeviceInput {
  address: String,
  name: String,
}

static VAILD_CHARS: &str = "1234567890ABCDEF";

fn validate_mac_addr(raw_mac_addr: &str) -> HubbitSchemaResult<()> {
  if raw_mac_addr.len() != 17 {
    return Err(HubbitSchemaError::InvalidInput);
  }

  for (i, c) in raw_mac_addr.chars().enumerate() {
    if i % 3 == 2 {
      if c != ':' {
        return Err(HubbitSchemaError::InvalidInput);
      }
    } else if !VAILD_CHARS.contains(c) {
      return Err(HubbitSchemaError::InvalidInput);
    }
  }

  Ok(())
}
