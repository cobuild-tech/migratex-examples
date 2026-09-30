import ApplicationSerializer from './application';

export default class ProfileSerializer extends ApplicationSerializer {
  primaryKey = 'username';

  normalizeFindRecordResponse(store, primaryModelClass, payload) {
    payload.profiles = payload.profile;
    delete payload.profile;
    return super.normalizeFindRecordResponse(...arguments);
  }
}
