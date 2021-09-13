import { gql } from 'urql';

export const getCurrentSessions = gql`
  query currentSessions {
    currentSessions {
      user {
        id
        nick
        avatarUrl
        groups
      }
      startTime
    }
  }
`;

export interface ActiveSessions {
  currentSessions: {
    user: {
      id: string;
      nick: string;
      avatarUrl: string;
      groups: string[];
    };
    startTime: Date;
  }[];
}
