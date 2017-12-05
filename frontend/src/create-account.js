import React, { Component } from 'react';
import { withRouter, Redirect } from 'react-router';
import PropTypes from 'prop-types';
import config from './config';

const steps = {
  error: 'error',
  default: 'default',
  creating: 'creating',
  created: 'created',
};

export class CreateAccount extends Component {
  constructor(props) {
    super(props);
    const pu = this.props.location.state && this.props.location.state.partialUser;
    this.state = {
      step: steps.default,
      email: '',
      username: '',
      partialUser: pu,
    };
    this.handleCreate = this.handleCreate.bind(this);
    this.handleInputChange = this.handleInputChange.bind(this);
  }

  handleInputChange(event) {
    this.setState({
      [event.target.name]: event.target.value,
    });
  }

  handleCreate() {
    this.setState(() => ({
      step: steps.creating,
    }));
    // validate
    if (this.state.email === '') {
      this.setState(() => ({
        step: steps.error,
        err: 'email is required',
      }));
      return;
    }
    if (this.state.username === '') {
      this.setState(() => ({
        step: steps.error,
        err: 'username is required',
      }));
      return;
    }

    fetch(`${config.api}/user/create`, {
      mode: 'cors',
      headers: { 'content-type': 'application/json' },
      credentials: 'include',
      method: 'POST',
      body: JSON.stringify({
        username: this.state.username,
        email: this.state.email,
        partial_user: this.state.partialUser,
      }),
    }).then(resp => resp.json()).then((resp) => {
      if (resp.Ok && resp.Ok.uuid) {
        this.setState({
          step: steps.created,
          uuid: resp.Ok.uuid,
          username: resp.Ok.username,
        });
      } else {
        this.setState({
          step: steps.error,
          err: (resp.Err && resp.Err.message) ? resp.Err.message : 'unknown error',
        });
      }
    });
  }

  render() {
    switch (this.state.step) {
      case steps.error:
        return (
          <div className="error">
            <h2> Error </h2>
            <p> {this.state.err} </p>
            <button onClick={() => { this.setState({ step: steps.default }); }}>
              Retry
            </button>
          </div>
        );
      case steps.default:
        if (this.state.partialUser) {
          const pu = this.state.partialUser;
          return (
            <div className="create-account">
              <p>
                Complete this form to create a Wobscale Account associated
                with your {pu.provider} account {`'${pu.provider_name}'`}.
              </p>
              <form onSubmit={this.handleCreate}>
                <label htmlFor="username">
                    Username:
                    <input
                      type="text"
                      onChange={this.handleInputChange}
                      name="username"
                    />
                </label>
                <br />

                <label htmlFor="email">
                    Email:
                    <input
                      type="text"
                      onChange={this.handleInputChange}
                      name="email"
                    />
                </label>
                <br />

                <input type="submit" value="Create" />
              </form>
            </div>
          );
        }
        // user got here without having a partial user, presumably this
        // is a browser history or f5 or such. Force em through the flow again.
        return <Redirect to="/" />;
      case steps.creating:
        return (
          <div className="logging-in">
            <p>Creating account...</p>
          </div>
        );
      case steps.created:
        return (
          <div className="created-in">
            <p>Created account {this.state.username}.</p>
            <p>
              Congratulations, you&quot;re the first user to get the UUID {this.state.uuid}!
            </p>
          </div>
        );
      default:
        return <Redirect to="/" />;
    }
  }
}
CreateAccount.propTypes = {
  location: PropTypes.object.isRequired,
};

export default withRouter(CreateAccount);
