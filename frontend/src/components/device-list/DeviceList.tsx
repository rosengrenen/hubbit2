import React, { useState } from 'react';

import { faTrashAlt } from '@fortawesome/free-regular-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { gql } from '@urql/core';

import { DeviceFragment, useSetDevicesMutation } from '../../__generated__/graphql';

import styles from './DeviceList.module.scss';

export const DEVICE_FRAGMENT = gql`
  fragment Device on Device {
    id
    address
    name
    isActive
  }
`;

gql`
  mutation SetDevices($input: SetDevicesInput!) {
    setDevices(data: $input) {
      ...Device
    }
  }

  ${DEVICE_FRAGMENT}
`;

interface Props {
  initialDevices: DeviceFragment[];
}

interface EditableDevice {
  address: string;
  description: string;
  savedAddress: string;
  savedDescription: string;
  isNew: boolean;
  isActive: boolean;
  unsavedChanges: boolean;
}

const DeviceList = ({ initialDevices }: Props) => {
  const [devices, setDevices] = useState(initState(initialDevices));

  const [, updateDevices] = useSetDevicesMutation();

  return (
    <div className={styles.macAddressesWrapper}>
      <table className="data-table card-shadow">
        <thead>
          <tr className="header-row">
            <th>Active</th>
            <th>MAC-Address</th>
            <th>Device Description</th>
            <th>Changed</th>
            <th />
          </tr>
        </thead>
        <tbody>
          {devices.map((device, index) => (
            <tr key={index}>
              <td className={styles.statusColumn}>
                <div
                  className={styles.statusIndicator + ` ${device.isActive ? styles.activeInHub : styles.inactiveInHub}`}
                />
              </td>
              <td>
                <input
                  className={styles.macTextField}
                  value={device.address}
                  onChange={e => {
                    // 16 hexadecimal chars + 7 dividing chars = 23
                    const val = e.target.value.substring(0, 23);
                    const newDevice = device;
                    newDevice.address = val;
                    newDevice.unsavedChanges = hasUnsavedChanges(newDevice);
                    setDevices(newDevice => [...newDevice]);
                  }}
                />
              </td>
              <td>
                <input
                  className={styles.macTextField}
                  value={device.description}
                  onChange={e => {
                    const val = e.target.value.substring(0, 40);
                    const newDevice = device;
                    newDevice.description = val;
                    newDevice.unsavedChanges = hasUnsavedChanges(newDevice);
                    setDevices(newDevice => [...newDevice]);
                  }}
                />
              </td>
              <td className={styles.changedCell}>{device.unsavedChanges ? '*' : ' '}</td>
              <td>
                <button
                  className={styles.iconButton}
                  onClick={() => {
                    if (window.confirm('Do you really want to delete this device?')) {
                      setDevices(devices => devices.filter((_device, i) => i !== index));
                    }
                  }}
                >
                  <FontAwesomeIcon icon={faTrashAlt} />
                </button>
              </td>
            </tr>
          ))}
        </tbody>
      </table>
      <div className={styles.helpText}>
        Don&apos;t know how to find your <a href={'https://en.wikipedia.org/wiki/MAC_address'}>MAC Address</a>? Take a
        look at <a href={'https://www.wikihow.com/Find-the-MAC-Address-of-Your-Computer'}>this guide</a>!
      </div>
      <button
        className={styles.saveButton}
        onClick={() => {
          setDevices(prev => [
            ...prev,
            {
              address: '',
              description: '',
              savedAddress: '',
              savedDescription: '',
              isActive: false,
              unsavedChanges: true,
              isNew: true,
            },
          ]);
        }}
      >
        Add device
      </button>
      <button
        className={styles.saveButton}
        disabled={initialDevices.length === devices.length && !devices.some(device => device.unsavedChanges)}
        onClick={() => {
          updateDevices({
            input: {
              devices: devices.map(device => ({
                address: device.address,
                name: device.description,
              })),
            },
          }).then(({ data }) => {
            // TODO(rasmus): if error show it to user
            if (data) {
              setDevices(initState(data.setDevices));
            }
          });
        }}
      >
        Save
      </button>
    </div>
  );
};

function hasUnsavedChanges(device: EditableDevice): boolean {
  return device.isNew || device.address !== device.savedAddress || device.description !== device.savedDescription;
}

function initState(devices: DeviceFragment[]): EditableDevice[] {
  return devices.map(device => ({
    address: device.address,
    description: device.name,
    savedAddress: device.address,
    savedDescription: device.name,
    isActive: device.isActive,
    isNew: false,
    unsavedChanges: false,
  }));
}

export default DeviceList;
