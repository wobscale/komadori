import { connect } from 'react-redux';
import { doBootstrapAdmin } from '../actions';
import BootstrapAdminReact from '../components/AdminBootstrap';

const mapStateToProps = (state) => {
  const { user } = state;
  return {
    user: user.user,
    token: '',
  };
};

const mapDispatchToProps = dispatch => ({
  adminBootstrap: (token) => {
    dispatch(doBootstrapAdmin(token));
  },
});

const BootstrapAdmin = connect(
  mapStateToProps,
  mapDispatchToProps,
)(BootstrapAdminReact);

export default BootstrapAdmin;
