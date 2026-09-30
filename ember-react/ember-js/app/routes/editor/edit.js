import Route from '@ember/routing/route';
import { action } from '@ember/object';
import { inject as service } from '@ember/service';

export default class EditorEditRoute extends Route {
  @service session;

  model({ id }) {
    return this.store.findRecord('article', id);
  }

  redirect(article) {
    let authorId = article.belongsTo('author').id();
    if (!this.session.user || authorId !== this.session.user.username) {
      this.transitionTo('articles.article', article.id);
    }
  }

  @action
  willTransition(transition) {
    let article = this.modelFor(this.routeName);
    if (article.hasDirtyAttributes && !article.isSaving) {
      if (
        window.confirm("You haven't saved your changes. Are you sure you want to leave the page?")
      ) {
        article.rollbackAttributes();
      } else {
        transition.abort();
      }
    }
  }
}
