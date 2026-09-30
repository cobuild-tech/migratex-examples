import Model, { attr } from '@ember-data/model';
import { inject as service } from '@ember/service';

export default class UserModel extends Model {
  @service session;

  @attr bio;
  @attr email;
  @attr image;
  @attr password;
  @attr token;
  @attr username;
  @attr('date') createdAt;
  @attr('date') updatedAt;

  async fetchFeed(page = 1, limit = 10) {
    let offset = (parseInt(page, 10) - 1) * limit;
    let { articles = [], articlesCount = 0 } = await this.session.fetch(
      `/articles/feed?limit=${limit}&offset=${offset}`,
    );
    let normalizedArticles = articles.map((article) =>
      Object.assign({}, article, { id: article.slug }),
    );
    this.store.pushPayload({ articles: normalizedArticles });
    let records = articles.map((article) => this.store.peekRecord('article', article.slug));
    records.meta = { articlesCount };
    return records;
  }
}
