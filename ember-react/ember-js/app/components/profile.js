import Component from '@glimmer/component';
import { inject as service } from '@ember/service';

export default class ProfileComponent extends Component {
  @service session;
}
