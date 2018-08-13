import React, { Component } from 'react';
import PropTypes from 'prop-types';

class UserDashboard extends Component {
  constructor(props) {
    super(props);
    const { user } = this.props;
    this.state = {
      user,
    };
  }

  render() {
    return (
      <div>
        <p>
          Welcome {this.state.user.username}!
        </p>
        <h3>Groups</h3>
        <p> {"You're"} in the following groups: </p>
        <ul>
          {
            this.state.user.groups.map(group => (<li key={group.uuid}>{group.name}</li>))
          }
        </ul>
      </div>
    );
  }
}
UserDashboard.propTypes = {
  user: PropTypes.object.isRequired,
};

export default UserDashboard;
