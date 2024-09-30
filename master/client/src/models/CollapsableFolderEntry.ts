import type { FolderEntry } from './FolderEntry'

/**
 * Collapsable folder entry element with nested elements
 */
export interface CollapsableFolderEntry extends FolderEntry {
  is_opened?: boolean
  nested?: CollapsableFolderEntry[]
}
