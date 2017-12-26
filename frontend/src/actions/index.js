import UserApi from '../user-api';

export const REQUEST_USER = 'REQUEST_USER';
export const RECEIVE_USER = 'RECEIVE_USER';
export const RECEIVE_NO_USER = 'RECEIVE_NO_USER';

export function requestUser() {
  return {
    type: REQUEST_USER,
  };
}

export function receiveUser(user) {
  return {
    type: RECEIVE_USER,
    receivedAt: Date.now(),
    user,
  };
}

export function receiveNoUser() {
  return {
    type: RECEIVE_NO_USER,
  };
}

export function getUser() {
  return (dispatch, getState) => {
    const { user } = getState();
    if (user.isFetching) {
      return Promise.resolve();
    }
    if (user.data && user.data.lastUpdated) {
      if (Date.now() - user.data.lastUpdated < 10 * 60 * 60 * 1000) {
        return Promise.resolve();
      }
    }
    return UserApi.get()
      .then((u) => {
        if (u.loggedIn) {
          dispatch(receiveUser(u.user));
        } else {
          dispatch(receiveNoUser());
        }
      }); // TODO: .catch
  };
}
