import React from 'react';
import PropTypes from 'prop-types';

const steps = {
  default: 'default',
  loggedIn: 'loggedIn',
  askConsent: 'askConsent',
  accept: 'accept',
  reject: 'reject',
  error: 'error',
};

export { steps as ConsentSteps };

const UserConsent = (props) => {
  switch (props.step) {
    case steps.default:
      return (
        <div>
          <h2>Authenticate with your Wobscale Account</h2>
          <p> Hang tight, we&#39;re checking if you&#39;re logged in. </p>
        </div>
      );
    case steps.loggedIn:
      props.getConsentInfo(props.consent.id);
      return (
        <div>
          <h2> Hello {props.user.username} </h2>
          <p> Please wait a moment while we figure out what {"you're"} authorizing to. </p>
        </div>
      );
    case steps.askConsent:
      return (
        <div>
          <h2> Hello {props.user.username} </h2>
          <p>
            {props.consent.client} would like permission to access:
          </p>
          <ul>
            {
              props.consent.scopes.map(el => (<li key={el}>{el}</li>))
            }
          </ul>
          <button onClick={() => props.onAccept(props.consent.id, props.consent.scopes)}>
            Accept
          </button>
          <button onClick={() => props.onReject(props.consent.id)}>
            Reject
          </button>
        </div>
      );
    case steps.accept:
      return (
        <div>
          <p> Hang tight, accepting... </p>
        </div>
      );
    case steps.reject:
      return (
        <div>
          <p> Hang tight, rejecting... </p>
        </div>
      );
    case steps.error:
      return (
        <div>
          <h2> Error </h2>
          <p> {props.err} </p>
        </div>
      );
    default:
      return (
        <div>
          This shouldn&quot;t happen
        </div>
      );
  }
};
UserConsent.propTypes = {
  step: PropTypes.string.isRequired,
  user: PropTypes.object,
  consent: PropTypes.object,
  onAccept: PropTypes.func.isRequired,
  onReject: PropTypes.func.isRequired,
  getConsentInfo: PropTypes.func.isRequired,
  err: PropTypes.string,
};
UserConsent.defaultProps = {
  err: '',
  user: null,
  consent: null,
};

export default UserConsent;
