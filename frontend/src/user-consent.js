import React, { Component } from 'react';
import PropTypes from 'prop-types';
import qs from 'query-string';

import UserAPI from './api/user';
import HydraAPI from './api/hydra';

const steps = {
  default: 'default',
  loggedIn: 'loggedIn',
  askConsent: 'askConsent',
  accept: 'accept',
  reject: 'reject',
  error: 'error',
};

class UserConsent extends Component {
  constructor(props) {
    super(props);
    const q = qs.parse(this.props.location.search);
    if (!q.consent) {
      this.state = {
        step: steps.error,
        err: "The 'consent' query parameter was not present; this shouldn't happen",
      };
      return;
    }

    this.state = {
      consentId: q.consent,
      user: null,
      step: steps.default,
    };
  }

  componentDidMount() {
    let { user } = this.state;
    if (!user && this.props.location.state && this.props.location.state.user) {
      user = this.props.location.state.user; // eslint-disable-line prefer-destructuring
    }

    const userPromise = user ? Promise.resolve(user) : UserAPI.get();

    userPromise.then((u) => {
      this.setState({
        step: steps.loggedIn,
        user: u,
      });

      return HydraAPI.getConsent(this.state.consentId);
    }).then((consent) => {
      this.setState({
        step: steps.askConsent,
        consent,
      });
    });
  }

  accept() {
    HydraAPI.acceptConsent(this.state.consentId, this.state.consent.scopes).then(() => {
      window.location = this.state.consent.redirect;
    });
  }

  reject() {
    HydraAPI.rejectConsent(this.state.consentId, 'User clicked reject').then(() => {
      window.location = this.state.consent.redirect;
    });
  }

  render() {
    switch (this.state.step) {
      case steps.default:
        return (
          <div>
            <h2>Authenticate with your Wobscale Account</h2>
            <p> Hang tight, we&#39;re checking if you&#39;re logged in. </p>
          </div>
        );
      case steps.loggedIn:
        return (
          <div>
            <h2> Hello {this.state.user.username} </h2>
            <p> Please wait a moment while we figure out what {"you're"} authorizing to. </p>
          </div>
        );
      case steps.askConsent:
        return (
          <div>
            <h2> Hello {this.state.user.username} </h2>
            <p>
              {this.state.consent.client} would like permission to access:
            </p>
            <ul>
              {
                this.state.consent.scopes.map(el => (<li key={el}>{el}</li>))
              }
            </ul>
            <button onClick={() => { this.setState({ step: steps.accept }); }}>
              Accept
            </button>
            <button onClick={() => { this.setState({ step: steps.reject }); }}>
              Reject
            </button>
          </div>
        );
      case steps.accept:
        this.accept();
        return (
          <div>
            <p> Hang tight, accepting... </p>
          </div>
        );
      case steps.reject:
        this.reject();
        return (
          <div>
            <p> Hang tight, rejecting... </p>
          </div>
        );
      case steps.error:
        return (
          <div>
            <h2> Error </h2>
            <p> {this.state.err} </p>
          </div>
        );
      default:
        return (
          <div>
            This shouldn&quot;t happen
          </div>
        );
    }
  }
}

UserConsent.propTypes = {
  location: PropTypes.object.isRequired,
};

export default UserConsent;
