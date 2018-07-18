import React, { Component } from 'react';
import PropTypes from 'prop-types';

class AdminManageUsers extends Component {
  componentDidMount() {
    this.props.adminListUsers();
  }

  render() {
    if (!this.props.users) {
      return <h3>Loading...</h3>;
    }
    return (
      <div>
        <h3>Users</h3>
        <span>TODO: provide link for each user to admin-manage page</span>
        <ul>
          {
            this.props.users.map(u => <li>{u.username}</li>)
          }
        </ul>
      </div>
    );
  }
}
AdminManageUsers.propTypes = {
  users: PropTypes.object,
  adminListUsers: PropTypes.func.isRequired,
};
AdminManageUsers.defaultProps = {
  users: null,
};

export default AdminManageUsers;
