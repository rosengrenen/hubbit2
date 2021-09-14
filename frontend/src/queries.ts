import { gql } from '@urql/core';

export const CURRENT_SESSIONS_QUERY = gql`
  query CurrentSessions {
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

export const ALL_TIME_SESSIONS_QUERY = gql`
  query AllTimeSessions {
    stats {
      user {
        nick
      }
      time
    }
  }
`;
