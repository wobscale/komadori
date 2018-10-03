import { connect } from 'react-redux';
import { doHandleAuth } from '../actions';
import DevLogin from '../components/DevLogin';

const mapDispatchToProps = dispatch => ({
  onAuth: (id) => {
    // the dev provider is mostly faked out; it just encodes data in 'state'
    // rather than doing real oauth stuff.
    const userInfo = {
      code: 'code',
      state: `${id} dev access_token`,
    };
    dispatch(doHandleAuth('local', userInfo));
  },
});

const mapStateToProps = () => ({
  authId: Math.floor(Math.random() * 420),
});

const LoginWithDev = connect(
  mapStateToProps,
  mapDispatchToProps,
)(DevLogin);

export default LoginWithDev;
