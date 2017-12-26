import React from 'react';
import ReactDOM from 'react-dom';
import { Provider } from 'react-redux';
import {
  BrowserRouter as Router,
  Route,
} from 'react-router-dom';
import thunkMiddleware from 'redux-thunk';
import { createStore, applyMiddleware } from 'redux';
import './index.css';
import App from './containers/App';
import registerServiceWorker from './registerServiceWorker';

import loginApp from './reducers';
import GithubLogin from './github-login-component';
import GithubOauthWindow from './github-oauth-window';

const store = createStore(
  loginApp,
  applyMiddleware(thunkMiddleware),
);

ReactDOM.render(
  <Provider store={store}>
    <Router>
      <div>
        <Route
          exact
          path="/"
          component={App}
        />
        <Route exact path="/github/login" component={GithubLogin} />
        <Route exact path="/github/oauth" component={GithubOauthWindow} />
      </div>
    </Router>
  </Provider>,
  document.getElementById('root'),
);
registerServiceWorker();
