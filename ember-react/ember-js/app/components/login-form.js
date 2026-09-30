import Component from '@glimmer/component';
import { tracked } from '@glimmer/tracking';
import { inject as service } from '@ember/service';
import { action } from '@ember/object';

export default class LoginFormComponent extends Component {
  @tracked email = '';
  @tracked password = '';
  @tracked user = null;
  @tracked loginErrors = [];

  @service session;
  @service router;

  @action
  async submit(e) {
    e.preventDefault();
    this.loginErrors = [];
    let result = await this.session.logIn(this.email, this.password);
    if (Array.isArray(result.errors)) {
      this.loginErrors = result.errors;
    } else {
      this.user = result;
      this.router.transitionTo('index');
    }
  }
}
