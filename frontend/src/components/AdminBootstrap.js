import React from 'react';
import PropTypes from 'prop-types';

class AdminBootstrapPage extends React.Component {
  constructor(props) {
    super(props);
    this.state = { token: '' };
  }
  render() {
    return (
      <div className="WobscaleAccounts">
        <p>
          Hello {this.props.user.username} <br />
          Are you an admin? Do you want to be? Just enter your bootstrap token!
        </p>
        <label htmlFor="token">
          Token:
          <input
            type="text"
            name="token"
            value={this.state.token || ''}
            onChange={evt => this.setState({ token: evt.target.value })}
          />
        </label>
        <button
          type="button"
          onClick={() => this.props.adminBootstrap(this.state.token ||
        '')}
        >
          Bootstrap me
        </button>
      </div>
    );
  }
}
AdminBootstrapPage.propTypes = {
  adminBootstrap: PropTypes.func.isRequired,
  user: PropTypes.object.isRequired,
};

export default AdminBootstrapPage;
