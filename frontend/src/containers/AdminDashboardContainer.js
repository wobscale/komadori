import { connect } from 'react-redux';
import { doAdminListUsers } from '../actions';
import AdminManageUsersComponent from '../components/AdminManageUsers';

const mapStateToProps = state => ({
  users: state.admin.users,
});

const mapDispatchToProps = dispatch => ({
  adminListUsers: () => {
    dispatch(doAdminListUsers());
  },
});

const AdminManageUsers = connect(
  mapStateToProps,
  mapDispatchToProps,
)(AdminManageUsersComponent);

export default AdminManageUsers;
