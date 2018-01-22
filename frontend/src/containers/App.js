import React, { Component } from 'react';
import {
  Route,
  Switch,
  Redirect,
  withRouter,
} from 'react-router-dom';
import PropTypes from 'prop-types';
import { connect } from 'react-redux';
import { doGetUser } from '../actions';
import UserDashboard from '../user-dashboard';
import LoginPage from '../components/LoginPage';
import GithubLogin from './LoginWithGithubContainer';
import GithubOauthWindow from '../github-oauth-window';
import UserConsent from '../user-consent';
import CreateAccount from '../create-account';


class ReactApp extends Component {
  componentDidMount() {
    const { dispatch } = this.props;
    dispatch(doGetUser());
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
      return (
        <div>
          <Route path="/" render={() => <Redirect to="/user/dashboard" />} />
          <Route
            path="/user/dashboard"
            render={props => (
              <UserDashboard {...props} user={user.user} />
            )}
          />
        </div>
      );
    }

    // Logged out routes
    return (
      <div>
        <Switch>
          <Route
            exact
            path="/"
            component={LoginPage}
          />
          <Route
            path="/"
            render={() => <Redirect to="/" />}
          />
        </Switch>
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
        { /* TODO: remember previous url to redirect back to */ }
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
