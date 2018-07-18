import React, { Component } from 'react';
import {
  Route,
  Switch,
  withRouter,
} from 'react-router-dom';
import PropTypes from 'prop-types';
import { connect } from 'react-redux';
import { doGetUser } from '../actions';
import GithubOauthWindow from '../github-oauth-window';
import {
  Login, CreateAccount, GithubLogin,
  UserDashboard, UserConsent,
  NavWrapper, BootstrapAdmin,
} from './AuthedContainers';

class ReactApp extends Component {
  componentDidMount() {
    const { dispatch } = this.props;
    dispatch(doGetUser());
  }

  render() {
    const {
      loading,
    } = this.props;

    if (loading) {
      return (
        <div>
          <h2>Loading</h2>
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
            component={Login}
          />
          <Route
            path="/github/oauth"
            component={GithubOauthWindow}
          />
          <Route
            path="/github/login"
            component={GithubLogin}
          />
          <Route
            path="/account/create"
            component={CreateAccount}
          />
          <Route
            path="/user/dashboard"
            render={props => <NavWrapper {...props} user={this.props.user.user} />}
          />
          <Route
            path="/user/consent"
            component={UserConsent}
          />
          <Route
            path="/admin/"
            render={props => <NavWrapper {...props} user={this.props.user.user} />}
          />
          <Route
            path="/admin/bootstrap"
            component={BootstrapAdmin}
          />
        </Switch>
      </div>
    );
  }
}
ReactApp.propTypes = {
  loading: PropTypes.bool.isRequired,
  dispatch: PropTypes.func.isRequired,
  user: PropTypes.object,
};
ReactApp.defaultProps = {
  user: null,
};

const mapStateToProps = (state) => {
  const { user, partialUser } = state;
  return {
    loading: !user.loaded,
    loggedIn: user.loggedIn,
    user,
    partialUser,
  };
};

const App = connect(
  mapStateToProps,
  null,
)(ReactApp);

export default withRouter(App);
