import { combineReducers } from 'redux';
import '../actions';

function handleUserState(state = { isFetching: false, loaded: false, loggedIn: false }, action) {
  switch (action.type) {
    case REQUEST_USER:
      return Object.assign({}, state, {
        isFetching: true,
        loaded: false,
      });
    case RECEIVE_USER:
      return Object.assign({}, state, {
        isFetching: false,
        user: action.user,
        loaded: true,
        loggedIn: true,
        lastUpdated: action.receivedAt,
      });
    case RECEIVE_NO_USER:
      return Object.assign({}, state, {
        isFetching: false,
        loaded: true,
        loggedIn: false,
      });
    default:
      return state;
  }
}

function handleConsentState(state = { isFetching: false, loaded: false, accepting: false, rejecting: false }, action) {
  switch (action.type) {
    case REQUEST_CONSENT_INFO:
      return Object.assign({}, state, {
        isFetching: true,
        loaded: false,
      });
  }
}


const rootReducer = combineReducers({
  user: handleUserState,
});

export default rootReducer;
