import React from 'react';
import PropTypes from 'prop-types';
import {
  BrowserRouter as Router,
  Route,
} from 'react-router-dom';

import LoginPage from './components/LoginPage';
import GithubLogin from './github-login-component';
import GithubOauthWindow from './github-oauth-window';
import UserDashboard from './user-dashboard';
import UserConsent from './user-consent';
import { CreateAccount } from './create-account';

import './App.css';


const MainPage = (props) => {
  if (props.user) {
    return <UserDashboard user={props.user} />;
  }
  return <LoginPage />;
};
MainPage.propTypes = {
  user: PropTypes.object.isRequired,
};

const App = () => (
  <div>
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
          path="/user/consent"
          render={props => <UserConsent {...props} />}
        />
      </div>
    </Router>
  </div>
);

export default App;
