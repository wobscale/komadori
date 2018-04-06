import { combineReducers } from 'redux';
import * as a from '../actions';

function handleUserState(state = { isFetching: false, loaded: false, loggedIn: false }, action) {
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
});

export default rootReducer;
