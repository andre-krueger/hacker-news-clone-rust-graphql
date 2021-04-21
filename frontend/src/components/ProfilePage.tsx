import React, {useEffect} from 'react';
import Text from './Text';
import {graphql, usePreloadedQuery} from 'react-relay';
import {useLazyLoadQuery} from 'react-relay/hooks';
import {ProfilePageQuery} from './__generated__/ProfilePageQuery.graphql';

// const UserQuery = graphql`
//   query ProfilePageQuery {
//     user(id: 1) {
//       id
//     }
//   }
// `;

function ProfilePage(props) {
  const data = useLazyLoadQuery<ProfilePageQuery>(
    graphql`
      query ProfilePageQuery {
        user(id: 1) {
          role
        }
      }
    `,
  );

  // if (isLoading) {
  //   return null;
  // }
  return <Text>{data.user.role}</Text>;
  return <Text>Profile</Text>;
}

export default ProfilePage;
