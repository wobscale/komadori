import React, { Component } from 'react';
import {
  BrowserRouter as Router,
  Route,
  Link
} from 'react-router-dom'
import config from './config';
import qs from 'query-string';

class GithubLogin extends Component {
    constructor(props) {
        super(props);
        this.state = {
            loggingIn: false,
        };
        this.loginClick = this.loginClick.bind(this);
    }

    loginClick() {
        this.setState(prev => ({
            loggingIn: true,
        }));
        // Stolen partially from 
        // https://github.com/rust-lang/crates.io/blob/e8cae0e872be4edf02f0876db4e85c082e70ecc9/app/routes/login.js#L14-L40
        // Used and modified under the terms of the MIT license.

        // TODO:
        // 2. retrieve oauth response from the window
        // 3. Get the user
        // 4. Route to dashboard or create user depending on 3.'s result
        window.github_response = undefined;
        let windowDimensions = [
            'width=1000',
            'height=450',
            'toolbar=0',
            'scrollbars=1',
            'status=1',
            'resizable=1',
            'location=1',
            'menuBar=0'
        ].join(',');
        let win = window.open("/github/oauth", "Github Authorization", windowDimensions);

        let waitClosed = window.setInterval(() => {
          if(!win.closed) {
            return;
          }
          window.clearInterval(waitClosed);
          if (window.github_response) {
            console.log(window.github_response);
          }
          this.setState(prev => ({
              loggingIn: false,
          }));
        }, 200);
    }

    render() {
        if(this.state.loggingIn) {
            return (
                <div className="logging-in">
                    <p>Please complete the login in the new window...</p>
                </div>
            );
        } else {
            return (
                <div>
                    <button onClick={this.loginClick}>
                        Login with Github
                    </button>
                </div>
            );
        }
    }
};

class GithubOauthWindow extends Component {
    state = { error: null };

    componentDidMount() {
      let q = qs.parse(this.props.location.search);
      if(q.code) {
        window.opener.github_response = {
          code: q.code,
          state: q.state,
        };
        window.close();
      }
      fetch(config.api + "/github/authorize_url", {credentials: 'include'})
        .then((res) => {
          if(res.status > 300) {
            throw new Error(res.statusText);
          }
          return res.text();
        })
        .then((url) => {
          window.location = url;
        })
        .catch((err) => {
            this.setState({error: err.toString()});
        });
    }

    render() {
      if(this.state.error) {
        return (
          <div>
            <p>An error occurred: {this.state.error}</p>
          </div>
        );
      } else {
        return (
          <p>Redirecting to Github Oauth Login</p>
        );
      }
    }
};

export { GithubOauthWindow };

export default GithubLogin;
