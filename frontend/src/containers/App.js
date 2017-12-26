import React, { Component } from 'react';
import PropTypes from 'prop-types';
import { connect } from 'react-redux';
import { getUser } from '../actions';
import UserDashboard from '../user-dashboard';
import LoginPage from '../components/LoginPage';


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
    return <LoginPage />;
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

export default App;
