import { Hono } from 'hono';

export const boardRouter = new Hono();

boardRouter.get('/', (c) => c.json({ status: 'mock board active' }));
