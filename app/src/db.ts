import { neon } from '@neondatabase/serverless';

const DATABASE_URL = process.env.DATABASE_URL;

export const sql = DATABASE_URL 
  ? neon(DATABASE_URL)
  : null as any;
