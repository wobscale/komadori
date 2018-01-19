import React, { Component } from 'react';
import {
  Route,
  withRouter,
} from 'react-router-dom';
import PropTypes from 'prop-types';
import { connect } from 'react-redux';
import { getUser } from '../actions';
import UserDashboard from '../user-dashboard';
import LoginPage from '../components/LoginPage';
import GithubLogin from '../github-login-component';
import GithubOauthWindow from '../github-oauth-window';
import UserConsent from '../user-consent';
import { CreateAccount } from '../create-account';


class ReactApp extends Component {
  componentDidMount() {
    const { dispatch } = this.props;
    dispatch(getUser());
  }

  render() {
    const { loading, loggedIn, user } = this.props;

    if (loading) {
      return (
        <div>
          <h2>Loading</h2>
        </div>
      );
    }
    if (loggedIn) {
      return <UserDashboard user={user.user} />;
    }

    // Logged out routes
    return (
      <div>
        <Route
          exact
          path="/"
          render={props => <LoginPage {...props} />}
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
    );
  }
}
ReactApp.propTypes = {
  loading: PropTypes.bool.isRequired,
  loggedIn: PropTypes.bool.isRequired,
  user: PropTypes.object.isRequired,
  dispatch: PropTypes.func.isRequired,
};

const mapStateToProps = (state) => {
  const { user } = state;
  return {
    loading: !user.loaded,
    loggedIn: user.loggedIn,
    user,
  };
};

const App = connect(
  mapStateToProps,
  null,
)(ReactApp);

export default withRouter(App);
