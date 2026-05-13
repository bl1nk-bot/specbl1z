import { Hono } from 'hono';
import { serve } from '@hono/node-server';
import { cors } from 'hono/cors';
import { logger } from 'hono/logger';
import { sql } from './db.js';
import { boardRouter } from './routes/board.js';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const publicDir = path.join(__dirname, '../public');

const app = new Hono();

app.use('*', logger());
app.use('*', cors());

app.get('/health', (c) => c.json({ status: 'ok' }));
app.route('/api/board', boardRouter);

app.get('/api/categories', async (c) => {
  if (!sql) return c.json([]);
  const cats = await sql`SELECT * FROM categories ORDER BY order_index ASC`;
  return c.json(cats);
});

app.get('/api/mcp/standards', async (c) => {
  if (!sql) return c.json({ error: 'DB not ready' });
  const standards = await sql`
    SELECT cat.label as category, s.title as section, r.text as rule, r.tag
    FROM rules r
    JOIN sections s ON r.section_id = s.id
    JOIN categories cat ON s.category_id = cat.id
    ORDER BY cat.order_index, s.order_index
  `;
  return c.json({ standards });
});

app.get('/', (c) => c.redirect('/public/index.html'));
app.get('/public/*', async (c) => {
  const filePath = path.join(publicDir, c.req.path.replace('/public/', ''));
  if (fs.existsSync(filePath) && fs.statSync(filePath).isFile()) {
    return c.body(fs.readFileSync(filePath));
  }
  return c.notFound();
});

serve({ fetch: app.fetch, port: 3000 });
