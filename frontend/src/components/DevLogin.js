import React from 'react';
import PropTypes from 'prop-types';

class DevLogin extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      authId: props.authId,
    };
    this.idInput = this.idInput.bind(this);
    this.handleSubmit = this.handleSubmit.bind(this);
  }

  idInput(ev) {
    this.setState({
      authId: ev.target.value,
    });
    ev.preventDefault();
  }

  handleSubmit(ev) {
    this.props.onAuth(this.state.authId);
    ev.preventDefault();
  }

  render() {
    return (
      <div>
        <h1>Wobscale Developer Login</h1>
        <p>Welcome to the wobscale developer login route.
        If this works in prod, something is deeply wrong!
          <small>It{"'"}s okay if it{"'"}s visible, just the buttons here shouldn{"'"}t work</small>
        </p>

        <form onSubmit={this.handleSubmit}>
          <label htmlFor="authId">User id: <input name="authId" id="authId" type="number" value={this.state.authId} onChange={this.idInput} /></label>
          <input type="submit" value="Dev Login As..." />
        </form>
      </div>
    );
  }
}
DevLogin.propTypes = {
  onAuth: PropTypes.func.isRequired,
  authId: PropTypes.number.isRequired,
};

export default DevLogin;

