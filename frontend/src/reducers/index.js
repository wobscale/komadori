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
}, action) {
  switch (action.type) {
    case a.REQUEST_CONSENT_INFO:
      return Object.assign({}, state, {
        isFetching: true,
        loaded: false,
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
