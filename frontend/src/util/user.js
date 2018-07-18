import conf from '../config';

export default class UserUtil {
  static userIsAdmin(user) {
    if (user == null) return false;

    return user.groups.map(u => u.uuid).includes(conf.adminGroup);
  }
}
