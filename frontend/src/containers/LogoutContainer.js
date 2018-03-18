import { connect } from 'react-redux';
import LogoutLink from '../components/LogoutLink';
import { doUserLogout } from '../actions';

const mapDispatchToProps = dispatch => ({
  onClick: () => {
    dispatch(doUserLogout());
  },
});


const Logout = connect(
  null,
  mapDispatchToProps,
)(LogoutLink);

export default Logout;
