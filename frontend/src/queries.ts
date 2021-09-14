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

export const STATS_QUERY = gql`
  query Stats($input: StatsInput) {
    stats(input: $input) {
      user {
        nick
      }
      time
    }
  }
`;
