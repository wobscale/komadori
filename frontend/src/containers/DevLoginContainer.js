import { connect } from 'react-redux';
import { doHandleAuth } from '../actions';
import DevLogin from '../components/DevLogin';

const mapDispatchToProps = dispatch => ({
  onAuth: (devInfo) => {
    dispatch(doHandleAuth('dev', devInfo));
  },
});

const LoginWithDev = connect(
  null,
  mapDispatchToProps,
)(DevLogin);

export default LoginWithDev;
