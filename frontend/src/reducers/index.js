import { combineReducers } from 'redux';
import * as a from '../actions';

const defaultUserState = { isFetching: false, loaded: false, loggedIn: false };

function handleUserState(state = defaultUserState, action) {
  switch (action.type) {
    case a.RECEIVE_USER:
      return Object.assign({}, state, {
        isFetching: false,
        user: action.user,
        loaded: true,
        loggedIn: true,
        lastUpdated: action.receivedAt,
      });
    case a.RECEIVE_NO_USER:
    case a.USER_LOGGED_OUT:
      return Object.assign({}, state, {
        isFetching: false,
        loaded: true,
        loggedIn: false,
      });
    case a.ADMIN_BOOTSTRAPPED:
      // Force a refetch since they're now an admin
      return defaultUserState;
    default:
      return state;
  }
}

function handleAdminState(state = {}, action) {
  switch (action.type) {
    case a.ADMIN_USER_LIST:
      return Object.assign({}, state, {
        users: action.data,
      });
    default:
      return state;
  }
}

function handlePartialUserState(state = { partialUser: null }, action) {
  switch (action.type) {
    case a.RECEIVE_PARTIAL_USER:
      return Object.assign({}, state, {
        partialUser: action.partialUser,
      });
    default:
      return state;
  }
}

function handleConsentState(state = {
  isFetching: false,
  loaded: false,
  accepting: false,
  rejecting: false,
  accepted: false,
  rejected: false,
}, action) {
  switch (action.type) {
    case a.REQUEST_CONSENT_INFO:
      return Object.assign({}, state, {
        isFetching: true,
        loaded: false,
      });
    case a.USER_CONSENT_FETCHED:
      return Object.assign({}, state, {
        isFetching: false,
        loaded: true,
        consent: action.data,
      });
    case a.USER_GIVE_CONSENT:
      return Object.assign({}, state, {
        isFetching: false,
        loaded: true,
        accepted: true,
        rejected: false,
      });
    case a.USER_REJECT_CONSENT:
      return Object.assign({}, state, {
        isFetching: false,
        loaded: true,
        accepted: false,
        rejected: true,
      });
    default:
      return state;
  }
}

const rootReducer = combineReducers({
  user: handleUserState,
  consent: handleConsentState,
  partialUser: handlePartialUserState,
  admin: handleAdminState,
});

export default rootReducer;
