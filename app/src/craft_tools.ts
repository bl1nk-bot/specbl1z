export const CRAFT_TOOLS = [
  // ==================== FOLDERS ====================
  {
    name: 'folders_list',
    description: 'List all folders in your Craft space. Returns folder IDs, names, and parent folder IDs.',
    inputSchema: {
      type: 'object',
      properties: {
        parentId: {
          type: 'string',
          description: 'Filter by parent folder ID to show only subfolders (optional)',
        },
      },
    },
  },
  {
    name: 'folders_create',
    description: 'Create a new folder in your Craft space.',
    inputSchema: {
      type: 'object',
      properties: {
        name: {
          type: 'string',
          description: 'Folder name',
        },
        parentId: {
          type: 'string',
          description: 'Parent folder ID (optional, creates in root if omitted)',
        },
      },
      required: ['name'],
    },
  },
  {
    name: 'folders_move',
    description: 'Move a folder to a new parent folder.',
    inputSchema: {
      type: 'object',
      properties: {
        folderId: {
          type: 'string',
          description: 'Folder ID to move',
        },
        newParentId: {
          type: 'string',
          description: 'Destination parent folder ID',
        },
      },
      required: ['folderId', 'newParentId'],
    },
  },
  {
    name: 'folders_delete',
    description: 'Delete a folder. WARNING: This deletes all documents inside the folder.',
    inputSchema: {
      type: 'object',
      properties: {
        folderId: {
          type: 'string',
          description: 'Folder ID to delete',
        },
      },
      required: ['folderId'],
    },
  },

  // ==================== DOCUMENTS ====================
  {
    name: 'documents_list',
    description: 'List documents in a folder. Supports pagination via cursor.',
    inputSchema: {
      type: 'object',
      properties: {
        folderId: {
          type: 'string',
          description: 'Folder ID to list documents from (optional, lists root if omitted)',
        },
        cursor: {
          type: 'string',
          description: 'Pagination cursor for next page of results',
        },
        limit: {
          type: 'number',
          description: 'Maximum results to return (default: 20, max: 100)',
        },
      },
    },
  },
  {
    name: 'documents_create',
    description: 'Create a new document in a folder. Returns the new document ID.',
    inputSchema: {
      type: 'object',
      properties: {
        title: {
          type: 'string',
          description: 'Document title',
        },
        folderId: {
          type: 'string',
          description: 'Folder ID to create the document in (optional, creates in root if omitted)',
        },
      },
      required: ['title'],
    },
  },
  {
    name: 'documents_move',
    description: 'Move a document to a different folder.',
    inputSchema: {
      type: 'object',
      properties: {
        documentId: {
          type: 'string',
          description: 'Document ID to move',
        },
        newFolderId: {
          type: 'string',
          description: 'Destination folder ID',
        },
      },
      required: ['documentId', 'newFolderId'],
    },
  },
  {
    name: 'documents_delete',
    description: 'Delete a document permanently.',
    inputSchema: {
      type: 'object',
      properties: {
        documentId: {
          type: 'string',
          description: 'Document ID to delete',
        },
      },
      required: ['documentId'],
    },
  },

  // ==================== SEARCH ====================
  {
    name: 'documents_search',
    description: 'Full-text search across all documents in your Craft space. Returns matching documents with snippets and IDs.',
    inputSchema: {
      type: 'object',
      properties: {
        query: {
          type: 'string',
          description: 'Search query text',
        },
        cursor: {
          type: 'string',
          description: 'Pagination cursor for next page of results',
        },
        limit: {
          type: 'number',
          description: 'Maximum results (default: 20, max: 100)',
        },
      },
      required: ['query'],
    },
  },
  {
    name: 'document_search',
    description: 'Search for text within a specific document. Returns matching blocks with surrounding context.',
    inputSchema: {
      type: 'object',
      properties: {
        documentId: {
          type: 'string',
          description: 'Document ID to search within',
        },
        query: {
          type: 'string',
          description: 'Text to find within the document',
        },
      },
      required: ['documentId', 'query'],
    },
  },

  // ==================== BLOCKS (Content) ====================
  {
    name: 'blocks_get',
    description: 'Read the content of a document. Returns blocks in JSON or Markdown format. Use documentId="today" for Daily Note, "tomorrow" or "yesterday" for relative dates.',
    inputSchema: {
      type: 'object',
      properties: {
        documentId: {
          type: 'string',
          description: 'Document ID, or "today"/"tomorrow"/"yesterday" for Daily Notes',
        },
        format: {
          type: 'string',
          enum: ['json', 'markdown'],
          description: 'Output format (default: json)',
        },
      },
      required: ['documentId'],
    },
  },
  {
    name: 'blocks_add',
    description: 'Add individual blocks to a document. For rich content, prefer markdown_add instead.',
    inputSchema: {
      type: 'object',
      properties: {
        documentId: {
          type: 'string',
          description: 'Document ID to add blocks to',
        },
        blocks: {
          type: 'array',
          description: 'Array of block objects to add',
          items: { type: 'object' },
        },
        insertAt: {
          type: 'string',
          description: 'Position: "start", "end" (default), or a specific blockId',
        },
      },
      required: ['documentId', 'blocks'],
    },
  },
  {
    name: 'blocks_update',
    description: 'Update an existing block\'s content or type.',
    inputSchema: {
      type: 'object',
      properties: {
        blockId: {
          type: 'string',
          description: 'Block ID to update',
        },
        content: {
          type: 'string',
          description: 'New text content',
        },
        type: {
          type: 'string',
          description: 'New block type',
          enum: ['text', 'heading', 'code', 'quote', 'callout', 'image', 'table', 'file', 'page', 'richUrl', 'line', 'collection'],
        },
      },
      required: ['blockId'],
    },
  },
  {
    name: 'blocks_delete',
    description: 'Delete a block from a document.',
    inputSchema: {
      type: 'object',
      properties: {
        blockId: {
          type: 'string',
          description: 'Block ID to delete',
        },
      },
      required: ['blockId'],
    },
  },
  {
    name: 'blocks_move',
    description: 'Move a block to a different position or document.',
    inputSchema: {
      type: 'object',
      properties: {
        blockId: {
          type: 'string',
          description: 'Block ID to move',
        },
        targetDocumentId: {
          type: 'string',
          description: 'Destination document ID',
        },
        insertAt: {
          type: 'string',
          description: 'Where to insert: "start", "end", or a specific blockId',
        },
      },
      required: ['blockId', 'targetDocumentId'],
    },
  },
  {
    name: 'markdown_add',
    description: 'Add rich content to a document from Markdown string. Supports Craft extensions: callouts, highlights, captions, nested pages. CRITICAL: This is the primary tool for inserting content. Put the ENTIRE desired content into the markdown parameter — do NOT split into multiple calls. Use documentId="today" for Daily Note.',
    inputSchema: {
      type: 'object',
      properties: {
        documentId: {
          type: 'string',
          description: 'Document ID to add content to, or "today"/"tomorrow"/"yesterday" for Daily Note',
        },
        markdown: {
          type: 'string',
          description: 'Complete Markdown content to insert. Supports Craft-specific syntax for callouts, highlights, captions, nested pages.',
        },
        insertAt: {
          type: 'string',
          description: 'Where to insert: "start", "end" (default), or a specific blockId',
        },
      },
      required: ['documentId', 'markdown'],
    },
  },
  {
    name: 'blocks_revert',
    description: 'Revert a block to a previous version. NOTE: Only available from the edit-review widget.',
    inputSchema: {
      type: 'object',
      properties: {
        blockId: {
          type: 'string',
          description: 'Block ID to revert',
        },
      },
      required: ['blockId'],
    },
  },

  // ==================== COLLECTIONS ====================
  {
    name: 'collections_list',
    description: 'List all collections in your space or within a specific document.',
    inputSchema: {
      type: 'object',
      properties: {
        documentId: {
          type: 'string',
          description: 'Optional document ID to filter collections by document',
        },
      },
    },
  },
  {
    name: 'collections_create',
    description: 'Create a new collection (structured database) inside a document. Define the schema with properties up front.',
    inputSchema: {
      type: 'object',
      properties: {
        documentId: {
          type: 'string',
          description: 'Document ID where the collection will be created',
        },
        name: {
          type: 'string',
          description: 'Collection name',
        },
        schema: {
          type: 'array',
          description: 'Array of property definitions (max 20). Types: text, number, date, url, email, select, multi_select, relation, boolean, checkbox, etc.',
          items: {
            type: 'object',
            properties: {
              name: { type: 'string' },
              type: { type: 'string' },
              options: { type: 'array', items: { type: 'string' } },
            },
            required: ['name', 'type'],
          },
        },
      },
      required: ['documentId', 'name', 'schema'],
    },
  },
  {
    name: 'collectionSchema_get',
    description: 'Get the schema of a collection. ALWAYS call this before modifying items to understand required fields. Use format "json-schema-items" for item data shape.',
    inputSchema: {
      type: 'object',
      properties: {
        collectionId: {
          type: 'string',
          description: 'Collection ID',
        },
        format: {
          type: 'string',
          enum: ['json-schema-items', 'json-schema-properties'],
          description: 'Output format. Use "json-schema-items" to see the shape items must follow (default)',
        },
      },
      required: ['collectionId'],
    },
  },
  {
    name: 'collectionSchema_update',
    description: 'Modify a collection schema: add, update, or remove properties (columns).',
    inputSchema: {
      type: 'object',
      properties: {
        collectionId: {
          type: 'string',
          description: 'Collection ID to update',
        },
        addProperties: {
          type: 'array',
          description: 'Properties to add',
          items: {
            type: 'object',
            properties: {
              name: { type: 'string' },
              type: { type: 'string' },
              options: { type: 'array', items: { type: 'string' } },
            },
            required: ['name', 'type'],
          },
        },
        updateProperties: {
          type: 'array',
          description: 'Properties to rename or change options',
          items: { type: 'object' },
        },
        removeProperties: {
          type: 'array',
          items: { type: 'string' },
          description: 'Property names to remove',
        },
      },
      required: ['collectionId'],
    },
  },
  {
    name: 'collectionItems_add',
    description: 'Add one or more items (rows) to a collection. Items must match the collection schema. Call collectionSchema_get first to know the required shape.',
    inputSchema: {
      type: 'object',
      properties: {
        collectionId: {
          type: 'string',
          description: 'Collection ID to add items to',
        },
        items: {
          type: 'array',
          description: 'Array of item objects matching the collection schema',
          items: { type: 'object' },
        },
      },
      required: ['collectionId', 'items'],
    },
  },
  {
    name: 'collectionItems_get',
    description: 'Retrieve items from a collection. Supports pagination and optional filters.',
    inputSchema: {
      type: 'object',
      properties: {
        collectionId: {
          type: 'string',
          description: 'Collection ID to read from',
        },
        filter: {
          type: 'object',
          description: 'Filter object to narrow down items',
        },
        cursor: {
          type: 'string',
          description: 'Pagination cursor for next page',
        },
        limit: {
          type: 'number',
          description: 'Maximum items to return (default: 50)',
        },
      },
      required: ['collectionId'],
    },
  },
  {
    name: 'collectionItems_update',
    description: 'Update an existing item in a collection.',
    inputSchema: {
      type: 'object',
      properties: {
        itemId: {
          type: 'string',
          description: 'Item ID to update',
        },
        changes: {
          type: 'object',
          description: 'Property changes to apply to the item',
        },
      },
      required: ['itemId', 'changes'],
    },
  },
  {
    name: 'collectionItems_delete',
    description: 'Delete an item from a collection.',
    inputSchema: {
      type: 'object',
      properties: {
        itemId: {
          type: 'string',
          description: 'Item ID to delete',
        },
      },
      required: ['itemId'],
    },
  },

  // ==================== TASKS ====================
  {
    name: 'tasks_get',
    description: 'Get tasks from your Craft space. Use scope to filter: "active" (due today or overdue), "upcoming" (scheduled future), "inbox" (unscheduled), "logbook" (completed/canceled), or "document" (tasks in a specific document, requires documentId).',
    inputSchema: {
      type: 'object',
      properties: {
        scope: {
          type: 'string',
          enum: ['active', 'upcoming', 'inbox', 'logbook', 'document'],
          description: 'Task scope to query (default: "active")',
        },
        documentId: {
          type: 'string',
          description: 'Document ID (required when scope is "document")',
        },
        cursor: {
          type: 'string',
          description: 'Pagination cursor',
        },
        limit: {
          type: 'number',
          description: 'Maximum tasks to return',
        },
      },
    },
  },
  {
    name: 'tasks_add',
    description: 'Create a new task. Can be in Inbox, Daily Note, or any document. Supports scheduling, deadlines, and repeating patterns.',
    inputSchema: {
      type: 'object',
      properties: {
        title: {
          type: 'string',
          description: 'Task title',
        },
        documentId: {
          type: 'string',
          description: 'Document ID to add task to, or "today"/"tomorrow"/"yesterday" for Daily Notes. Omit for Inbox.',
        },
        due: {
          type: 'string',
          description: 'Due date (ISO 8601, or relative: "tomorrow", "next week")',
        },
        status: {
          type: 'string',
          enum: ['todo', 'done', 'canceled'],
          description: 'Task status (default: "todo")',
        },
        schedule: {
          type: 'string',
          description: 'Scheduled date/time',
        },
        repeat: {
          type: 'object',
          description: 'Repeating pattern configuration',
        },
      },
      required: ['title'],
    },
  },
  {
    name: 'tasks_update',
    description: 'Update a task — mark as done, reschedule, change title, etc.',
    inputSchema: {
      type: 'object',
      properties: {
        taskId: {
          type: 'string',
          description: 'Task ID to update',
        },
        title: {
          type: 'string',
          description: 'New title',
        },
        status: {
          type: 'string',
          enum: ['todo', 'done', 'canceled'],
          description: 'New status',
        },
        due: {
          type: 'string',
          description: 'New due date',
        },
        schedule: {
          type: 'string',
          description: 'New scheduled date/time',
        },
      },
      required: ['taskId'],
    },
  },
  {
    name: 'tasks_delete',
    description: 'Delete a task permanently.',
    inputSchema: {
      type: 'object',
      properties: {
        taskId: {
          type: 'string',
          description: 'Task ID to delete',
        },
      },
      required: ['taskId'],
    },
  },

  // ==================== COMMENTS ====================
  {
    name: 'comments_add',
    description: 'Add a comment to a block.',
    inputSchema: {
      type: 'object',
      properties: {
        blockId: {
          type: 'string',
          description: 'Block ID to comment on',
        },
        content: {
          type: 'string',
          description: 'Comment text',
        },
      },
      required: ['blockId', 'content'],
    },
  },

  // ==================== IMAGES ====================
  {
    name: 'image_view',
    description: 'View an image embedded in a document. Returns base64 content (max 1MB, image types only).',
    inputSchema: {
      type: 'object',
      properties: {
        blockId: {
          type: 'string',
          description: 'Block ID containing the image',
        },
      },
      required: ['blockId'],
    },
  },

  // ==================== CONNECTION ====================
  {
    name: 'connection_info',
    description: 'Get information about the current Craft connection: space ID, timezone, current time, and deep link templates.',
    inputSchema: {
      type: 'object',
      properties: {},
    },
  },
] as const;
