import UserApi from '../api/user';
import HydraAPI from '../api/hydra';

export const RECEIVE_USER = 'RECEIVE_USER';
export const RECEIVE_NO_USER = 'RECEIVE_NO_USER';
export const RECEIVE_PARTIAL_USER = 'RECEIVE_PARTIAL_USER';

function receivePartialUser(partialUser) {
  return {
    type: RECEIVE_PARTIAL_USER,
    partialUser,
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

export function doGetUser() {
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

export const USER_LOGOUT = 'USER_LOGOUT';
export const USER_LOGGED_OUT = 'USER_LOGGED_OUT';

export function userLogout() {
  return {
    type: USER_LOGOUT,
  };
}

export function userLoggedOut() {
  return {
    type: USER_LOGGED_OUT,
  };
}

export function doUserLogout() {
  return (dispatch, getState) => {
    const { user } = getState();
    if (!user.loggedIn) {
      throw new Error('cannot logout user: not logged in');
    }

    return UserApi.logout()
      .then(() => {
        dispatch(userLoggedOut());
      });
  };
}

export const REQUEST_CONSENT_INFO = 'REQUEST_CONSENT_INFO';
export const USER_CONSENT_FETCHING = 'USER_CONSENT_FETCHING';
export const USER_CONSENT_FETCHED = 'USER_CONSENT_FETCHED';
export const USER_GIVE_CONSENT = 'USER_GIVE_CONSENT';
export const USER_REJECT_CONSENT = 'USER_REJECT_CONSENT';
export const USER_GOT_CONSENT = 'USER_GOT_CONSENT';
export const USER_GOT_REJECT_CONSENT = 'USER_GOT_REJECT_CONSENT';

export function userConsentFetching() {
  return {
    type: USER_CONSENT_FETCHED,
  };
}

export function userConsentFetched() {
  return {
    type: USER_CONSENT_FETCHED,
  };
}

export function userGiveConsent() {
  return {
    type: USER_GIVE_CONSENT,
  };
}

export function userGotConsent() {
  return {
    type: USER_GOT_CONSENT,
  };
}

export function doGetConsent() {
  return (dispatch, getState) => {
    const { consent } = getState();
  };
}

export function doGiveConsent() {
  return (dispatch, getState) => {
    const { user, consentFlow } = getState();
    if (!user.loggedIn) {
      throw new Error('Must be logged in');
    }
    HydraAPI.acceptConsent(consentFlow.id, consentFlow.scopes)
      .then((consent) => {
      });
  };
}

export function doHandleAuth(provider, providerInfo) {
  return (dispatch) => {
    UserApi.auth(provider, providerInfo.code, providerInfo.state)
      .then((resp) => {
        if (resp.type === 'PartialUser') {
          // Needs to create an account
          dispatch(receivePartialUser(resp));
        } else if (resp.type === 'UserResp') {
          dispatch(receiveUser(resp));
        }
      });
  };
}

export function doCreateAccount(userInfo) {
}
