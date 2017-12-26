import { combineReducers } from 'redux';
import {
  REQUEST_USER,
  RECEIVE_USER,
  RECEIVE_NO_USER,
} from '../actions';

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


const rootReducer = combineReducers({
  user: handleUserState,
});

export default rootReducer;
