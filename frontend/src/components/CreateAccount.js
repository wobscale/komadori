import React, { Component } from 'react';
import { Redirect } from 'react-router';
import PropTypes from 'prop-types';

const steps = {
  error: 'error',
  default: 'default',
  creating: 'creating',
  created: 'created',
};

class CreateAccountReact extends Component {
  constructor(props) {
    super(props);
    const pu = props.partialUser;
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
    }

    this.props.createAccount(this.state);
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
        alert("Redirect");
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
          </div>
        );
      default:
        return (<b>No valid step, not sure what happened</b>);
    }
  }
}
CreateAccountReact.propTypes = {
  partialUser: PropTypes.object.isRequired,
  createAccount: PropTypes.func.isRequired,
};

export default CreateAccountReact;
