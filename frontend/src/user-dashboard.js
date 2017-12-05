import React, { Component } from 'react';
import PropTypes from 'prop-types';

class UserDashboard extends Component {
  constructor(props) {
    super(props);
    const user = this.props.location.state && this.props.location.state.user;
    this.state = {
      user,
    };
  }

  render() {
    return (
      <div>
        Welcome {this.state.user.username}!
      </div>
    );
  }
}
UserDashboard.propTypes = {
  location: PropTypes.object.isRequired,
};

export default UserDashboard;
