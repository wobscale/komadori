import UserApi from '../user-api';
import HydraAPI from '../hydra-api';

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

const REQUEST_CONSENT_INFO = 'REQUEST_CONSENT_INFO';
const USER_CONSENT_FETCHING = 'USER_CONSENT_FETCHING';
const USER_CONSENT_FETCHED = 'USER_CONSENT_FETCHED';
const USER_GIVE_CONSENT = 'USER_GIVE_CONSENT';
const USER_REJECT_CONSENT = 'USER_REJECT_CONSENT';
const USER_GOT_CONSENT = 'USER_GOT_CONSENT';
const USER_GOT_REJECT_CONSENT = 'USER_GOT_REJECT_CONSENT';

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

export function userReceiveConsent() {
  return {
    type: USER_RECEIVE_CONSENT,
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
