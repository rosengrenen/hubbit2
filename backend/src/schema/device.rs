use async_graphql::{guard::Guard, Context, InputObject, Object};
use uuid::Uuid;

use crate::{
  models::GammaUser,
  repositories::device::{CreateDevice, DeviceRepository, UpdateDevice},
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
    let device = device_repo
      .get_by_id(self.id)
      .await
      .map_err(|_| HubbitSchemaError::InternalError)?;
    Ok(device.address)
  }

  async fn name(&self, context: &Context<'_>) -> HubbitSchemaResult<String> {
    let device_repo = context.data_unchecked::<DeviceRepository>();
    let device = device_repo
      .get_by_id(self.id)
      .await
      .map_err(|_| HubbitSchemaError::InternalError)?;
    Ok(device.name)
  }
}

#[derive(Default)]
pub struct DeviceMutation;

#[Object]
impl DeviceMutation {
  #[graphql(guard(AuthGuard()))]
  pub async fn create_device(
    &self,
    context: &Context<'_>,
    data: CreateDeviceInput,
  ) -> HubbitSchemaResult<Device> {
    let address = data.address.to_uppercase();
    validate_mac_addr(&address)?;
    let device_repo = context.data_unchecked::<DeviceRepository>();
    let auth_user = context.data_unchecked::<GammaUser>();
    let device = device_repo
      .create(CreateDevice {
        user_id: auth_user.id,
        address,
        name: data.name,
      })
      .await
      .map_err(|_| HubbitSchemaError::InternalError)?;
    Ok(Device { id: device.id })
  }

  #[graphql(guard(AuthGuard()))]
  pub async fn update_device(
    &self,
    context: &Context<'_>,
    id: Uuid,
    data: UpdateDeviceInput,
  ) -> HubbitSchemaResult<Device> {
    let device_repo = context.data_unchecked::<DeviceRepository>();
    let auth_user = context.data_unchecked::<GammaUser>();
    let device = device_repo
      .get_by_id(id)
      .await
      .map_err(|_| HubbitSchemaError::NotFound)?;
    if device.user_id != auth_user.id {
      return Err(HubbitSchemaError::NotAuthorized);
    }

    let device = device_repo
      .update(id, UpdateDevice { name: data.name })
      .await
      .map_err(|_| HubbitSchemaError::InternalError)?;
    Ok(Device { id: device.id })
  }

  #[graphql(guard(AuthGuard()))]
  pub async fn delete_device(&self, context: &Context<'_>, id: Uuid) -> HubbitSchemaResult<bool> {
    let device_repo = context.data_unchecked::<DeviceRepository>();
    let auth_user = context.data_unchecked::<GammaUser>();
    let device = device_repo
      .get_by_id(id)
      .await
      .map_err(|_| HubbitSchemaError::NotFound)?;
    if device.user_id != auth_user.id {
      return Err(HubbitSchemaError::NotAuthorized);
    }

    device_repo
      .delete(id)
      .await
      .map_err(|_| HubbitSchemaError::InternalError)?;
    Ok(true)
  }
}

#[derive(InputObject)]
pub struct CreateDeviceInput {
  address: String,
  name: String,
}

#[derive(InputObject)]
pub struct UpdateDeviceInput {
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
