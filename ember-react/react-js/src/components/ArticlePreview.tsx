import { Link } from 'react-router-dom';
import type { Article } from '../api/types';
import { ArticleAuthor } from './ArticleAuthor';
import { FavoriteArticle } from './FavoriteArticle';
import { articlePath } from '../lib/paths';

export function ArticlePreview({ article }: { article: Article }) {
  return (
    <div className="article-preview" data-test-article-preview={article.slug}>
      <div className="article-meta">
        <ArticleAuthor author={article.author} updatedAt={article.updatedAt} />
        <FavoriteArticle article={article} isIconOnly className="pull-xs-right" />
      </div>
      <Link to={articlePath(article.slug)} className="preview-link" data-test-article-title>
        <h1>{article.title}</h1>
        <p>{article.description}</p>
        <span>Read more...</span>
        {article.tagList.length > 0 && (
          <ul className="tag-list">
            {article.tagList.map((tag, index) => (
              <li key={index} className="tag-default tag-pill tag-outline">
                <span>{tag}</span>
              </li>
            ))}
          </ul>
        )}
      </Link>
    </div>
  );
}
