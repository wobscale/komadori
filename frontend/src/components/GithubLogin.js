import { withRouter } from 'react-router';
import {
  Redirect,
} from 'react-router-dom';
import React, { Component } from 'react';
import PropTypes from 'prop-types';

const steps = {
  error: 'error',
  loggingIn: 'loggingIn',
  verifyingLogin: 'verifyingLogin',
};

class GithubLogin extends Component {
  constructor(props) {
    super(props);
    this.state = {
      step: steps.loggingIn,
    };
  }

  startLoginFlow() {
    // Stolen partially from
    // https://github.com/rust-lang/crates.io/blob/e8cae0e872be4edf02f0876db4e85c082e70ecc9/app/routes/login.js#L14-L40
    // Used and modified under the terms of the MIT license.

    // 4. Route to dashboard or create user depending on 3.'s result
    window.github_response = undefined;
    const windowDimensions = [
      'width=1000',
      'height=450',
      'toolbar=0',
      'scrollbars=1',
      'status=1',
      'resizable=1',
      'location=1',
      'menuBar=0',
    ].join(',');
    const win = window.open('/github/oauth', 'Github Authorization', windowDimensions);

    new Promise((resolve, reject) => {
      const waitClosed = window.setInterval(() => {
        if (!win) {
          reject(new Error('null widow'));
          // popup blocked probably
          return;
        }
        if (!win.closed) {
          return;
        }
        window.clearInterval(waitClosed);
        if (window.github_response) {
          resolve(window.github_response);
        } else {
          reject(new Error('did not get github_response'));
        }
      }, 200);
    }).then((resp) => {
      this.props.onAuth(resp);
    });
  }

  render() {
    if (this.state.step === steps.loggingIn) {
      this.startLoginFlow();
      return (
        <div className="logging-in">
          <p>Please complete the login in the new window...</p>
        </div>
      );
    } else if (this.state.step === steps.verifyingLogin) {
      return (
        <div className="logging-in">
          <p>Verifying your login...</p>
        </div>
      );
    } else if (this.state.step === steps.error) {
      return (
        <div>
          <h2>An error occured</h2>
          <div className="error"> {this.state.err} </div>
          <button onClick={() => { this.setState({ step: steps.loggingIn }); }}>
            Retry
          </button>
        </div>
      );
    }
    return (
      <div>
        <p>Logged in, you&quot;ll be redirected shortly</p>
      </div>
    );
  }
}
GithubLogin.propTypes = {
  onAuth: PropTypes.func.isRequired,
};

export default withRouter(GithubLogin);
