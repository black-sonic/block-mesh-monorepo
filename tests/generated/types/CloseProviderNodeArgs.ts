/**
 * This code was GENERATED using the solita package.
 * Please DO NOT EDIT THIS FILE, instead rerun solita to update it or write a wrapper to add functionality.
 *
 * See: https://github.com/metaplex-foundation/solita
 */

import * as beet from '@metaplex-foundation/beet'
export type CloseProviderNodeArgs = {
  bump: number
}

/**
 * @category userTypes
 * @category generated
 */
export const closeProviderNodeArgsBeet =
  new beet.BeetArgsStruct<CloseProviderNodeArgs>(
    [['bump', beet.u8]],
    'CloseProviderNodeArgs'
  )
