import React from 'react';
import { withRouter } from 'react-router';
import {
  BrowserRouter as Router,
  Route,
  Link,
} from 'react-router-dom';

import GithubLogin from './github-login-component';
import GithubOauthWindow from './github-oauth-window';
import UserDashboard from './user-dashboard';
import UserConsent from './user-consent';
import { CreateAccount } from './create-account';

import logo from './logo.svg';
import './App.css';

const App = function () {
  return (
    <div className="WobscaleAccounts">
      <header className="Wobscale-Login">
        <img src={logo} className="Wobscale-logo" alt="logo" />
      </header>
      <Router>
        <div>
          <Route
            exact
            path="/"
            render={props => <MainPage {...props} />}
          />
          <Route
            path="/github/oauth"
            render={props => <GithubOauthWindow {...props} />}
          />
          <Route
            path="/github/login"
            render={props => <GithubLogin {...props} />}
          />
          <Route
            path="/account/create"
            render={props => <CreateAccount {...props} />}
          />
          <Route
            path="/user/dashboard"
            render={props => <UserDashboard {...props} />}
          />
          <Route
            path="/user/consent"
            render={props => <UserConsent {...props} />}
          />
        </div>
      </Router>
    </div>
  );
};

const MainPage = withRouter(() => (
  <div>
    <h1 className="App-title">Login</h1>
    <p>
      Login to your Wobscale Account
    </p>
    <Link to="/github/login">
      <button type="button">
        Login with Github
      </button>
    </Link>

    <h2>Create an account</h2>
    <p> If you {"don't"} have an account yet, login to create one </p>
  </div>
));

export default App;
