import { connect } from 'react-redux';
import qs from 'query-string';

import UserConsentComponent, { ConsentSteps } from '../components/ConsentComponent';
import { doGetConsentInfo, doGiveConsent, doRejectConsent } from '../actions';

const mapStateToProps = (state, ownProps) => {
  const q = qs.parse(ownProps.location.search);
  if (!q.consent) {
    return {
      step: ConsentSteps.error,
      err: "The 'consent' query parameter was not present; this shouldn't happen",
    };
  }
  const { user, consent } = state;
  if (!consent.loaded) {
    return {
      step: ConsentSteps.loggedIn,
      user: user.user,
      consent: { id: q.consent },
    };
  }

  if (consent.accepted) {
    window.location = consent.consent.redirect;
  }
  if (consent.rejected) {
    window.location = consent.consent.redirect;
  }

  const step = ConsentSteps.askConsent;
  return {
    step,
    user: user.user,
    consent: consent.consent,
  };
};

const mapDispatchToProps = dispatch => ({
  getConsentInfo: (id) => {
    dispatch(doGetConsentInfo(id));
  },
  onAccept: (id, scopes) => {
    dispatch(doGiveConsent(id, scopes));
  },
  onReject: (id) => {
    dispatch(doRejectConsent(id, 'User clicked reject'));
  },
});

const UserConsent = connect(
  mapStateToProps,
  mapDispatchToProps,
)(UserConsentComponent);

export default UserConsent;
