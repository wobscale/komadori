import { withRouter } from 'react-router';
import {
  Redirect,
} from 'react-router-dom';
import React, { Component } from 'react';
import PropTypes from 'prop-types';
import config from './config';

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
      this.setState(() => ({
        step: steps.verifyingLogin,
      }));
      return fetch(`${config.api}/user/auth`, {
        mode: 'cors',
        headers: { 'content-type': 'application/json' },
        credentials: 'include',
        method: 'POST',
        body: JSON.stringify({
          provider: 'github',
          code: resp.code,
          state: resp.state,
        }),
      });
    }).then(resp => resp.json()).then((resp) => {
      if (resp.Ok && resp.Ok.type === 'PartialUser') {
        this.props.history.push({
          pathname: '/account/create',
          state: {
            partialUser: resp.Ok,
          },
        });
      } else if (resp.Ok && resp.Ok.type === 'UserResp') {
        this.props.history.push({
          pathname: '/user/dashboard',
          state: {
            user: resp.Ok,
          },
        });
      } else {
        this.setState(() => ({
          step: steps.error,
          err: 'Response did not include a user or partial user.',
        }));
      }
    })
      .catch((err) => {
        // TODO better error handling here, pff alerts
        this.setState(() => ({
          step: steps.error,
          err: err.toString(),
        }));
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
        <Redirect to="/" />
      </div>
    );
  }
}
GithubLogin.propTypes = {
  history: PropTypes.object.isRequired,
};

const githubLoginExport = withRouter(GithubLogin);
export default githubLoginExport;
