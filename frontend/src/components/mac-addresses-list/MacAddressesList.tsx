import React, { useState } from 'react';

import { faTrashAlt } from '@fortawesome/free-regular-svg-icons';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';

import { MeQuery } from '../../__generated__/graphql';

import styles from './MacAddressesList.module.scss';

interface props {
  initialDevices: MeQuery['me']['devices'];
}

interface EditableDevice {
  address: string;
  description: string;
  savedAddress: string;
  savedDescription: string;
  isNew: boolean;
  unsavedChanges: boolean;
}

const MacAddressesList = ({ initialDevices }: props) => {
  const [devices, setDevices] = useState(
    initialDevices.map(device => {
      return {
        address: device.address,
        description: device.name,
        savedAddress: device.address,
        savedDescription: device.name,
        isNew: false,
        unsavedChanges: false,
      };
    }),
  );

  return (
    <div className={styles.macAddressesWrapper}>
      <table className={'data-table card-shadow'}>
        <thead>
          <tr className={'header-row'}>
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
                  className={styles.statusIndicator + ` ${index % 2 === 0 ? styles.activeInHub : styles.inactiveInHub}`}
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
                  readOnly={false}
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
                      setDevices(devices.filter((dev, i) => i !== index));
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
        Don't know how to find your <a href={'https://en.wikipedia.org/wiki/MAC_address'}>MAC Address</a>? Take a look
        at <a href={'https://www.wikihow.com/Find-the-MAC-Address-of-Your-Computer'}>this guide</a>!
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
        disabled={!devices.some(device => device.unsavedChanges)}
        onClick={() => {
          // TODO(vidarm): Send the data to the server
          console.log('Not implemented');
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

export default MacAddressesList;
