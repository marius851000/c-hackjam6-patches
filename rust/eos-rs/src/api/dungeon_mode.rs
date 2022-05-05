//! Traits, structs and functions related to dungeon mode.

use core::ffi::CStr;
use core::fmt::Debug;
use crate::api::fixed::I24F8;
use crate::api::alias::{DungeonEntity, DungeonItem, DungeonMonster, DungeonMove, DungeonTrap, item_catalog};
use crate::api::alias::entity_type;
use crate::api::overlay::{CreatableWithLease, OverlayLoadLease};
use crate::ctypes::*;
use crate::{ctypes, ffi};
use crate::string_util::str_to_cstring;

/// Extension trait for `DungeonEntity`.
pub trait DungeonEntityExt {
    /// This returns the monster info struct for the entity,
    /// panics if the entity is not a monster.
    fn info_for_monster(&self) -> Option<&DungeonMonster>;

    /// This returns the item info struct for the entity,
    /// panics if the entity is not an item.
    fn info_for_item(&self) -> Option<&DungeonItem>;

    /// This returns the trap info struct for the entity,
    /// panics if the entity is not a trap.
    fn info_for_trap(&self) -> Option<&DungeonTrap>;

    /// This returns the monster info struct for the entity,
    /// panics if the entity is not a monster.
    fn info_for_monster_mut(&mut self) -> Option<&mut DungeonMonster>;

    /// This returns the item info struct for the entity,
    /// panics if the entity is not an item.
    fn info_for_item_mut(&mut self) -> Option<&mut DungeonItem>;

    /// This returns the trap info struct for the entity,
    /// panics if the entity is not a trap.
    fn info_for_trap_mut(&mut self) -> Option<&mut DungeonTrap>;
}

impl DungeonEntityExt for DungeonEntity {
    fn info_for_monster(&self) -> Option<&DungeonMonster> {
        if self.type_ == entity_type::ENTITY_MONSTER {
            unsafe { Some(&*(self.info as *const DungeonMonster)) }
        } else {
            None
        }
    }

    fn info_for_item(&self) -> Option<&DungeonItem> {
        if self.type_ == entity_type::ENTITY_ITEM {
            unsafe { Some(&*(self.info as *const DungeonItem)) }
        } else {
            None
        }
    }

    fn info_for_trap(&self) -> Option<&DungeonTrap> {
        if self.type_ == entity_type::ENTITY_TRAP {
            unsafe { Some(&*(self.info as *const DungeonTrap)) }
        } else {
            None
        }
    }

    fn info_for_monster_mut(&mut self) -> Option<&mut DungeonMonster> {
        if self.type_ == entity_type::ENTITY_MONSTER {
            unsafe { Some(&mut *(self.info as *mut DungeonMonster)) }
        } else {
            None
        }
    }

    fn info_for_item_mut(&mut self) -> Option<&mut DungeonItem> {
        if self.type_ == entity_type::ENTITY_ITEM {
            unsafe { Some(&mut *(self.info as *mut DungeonItem)) }
        } else {
            None
        }
    }

    fn info_for_trap_mut(&mut self) -> Option<&mut DungeonTrap> {
        if self.type_ == entity_type::ENTITY_TRAP {
            unsafe { Some(&mut *(self.info as *mut DungeonTrap)) }
        } else {
            None
        }
    }
}

/// Helper struct for emitting move and item effects.
pub struct DungeonEffectsEmitter(OverlayLoadLease<29>);

impl CreatableWithLease<29> for DungeonEffectsEmitter {
    fn _create(lease: OverlayLoadLease<29>) -> Self {
        Self(lease)
    }

    fn lease(&self) -> &OverlayLoadLease<29> {
        &self.0
    }
}

// overlay29:
//   versions:
//     - NA
//     - EU
//     - JP
//   address:
//     NA: 0x22DC240
//     EU: 0x22DCB80
//   length:
//     NA: 0x77620
//     EU: 0x77900
//   description: |-
//     The dungeon engine.
//
//     This is the "main" overlay of dungeon mode. It controls most things that happen in a Mystery Dungeon, such as dungeon layout generation, dungeon menus, enemy AI, and generally just running each turn while within a dungeon.
//   functions:
//     - name: DungeonAlloc
//       address:
//         NA: 0x22DEA5C
//         EU: 0x22DF39C
//       description: |-
//         Allocates a new dungeon struct.
//
//         This updates the master dungeon pointer and returns a copy of that pointer.
//
//         return: pointer to a newly allocated dungeon struct
//     - name: GetDungeonPtrMaster
//       address:
//         NA: 0x22DEA80
//       description: |-
//         Returns the master dungeon pointer (a global, see DUNGEON_PTR_MASTER).
//
//         return: pointer to a newly allocated dungeon struct
//     - name: DungeonZInit
//       address:
//         NA: 0x22DEA90
//         EU: 0x22DF3D0
//       description: |-
//         Zero-initializes the dungeon struct pointed to by the master dungeon pointer.
//
//         No params.
//     - name: DungeonFree
//       address:
//         NA: 0x22DEAB0
//       description: |-
//         Frees the dungeons struct pointer to by the master dungeon pointer, and nullifies the pointer.
//
//         No params.
//     - name: InitializeDungeon
//       address:
//         NA: 0x22DEF38
//         EU: 0x22DF878
//       description: Seems to initialize the dungeon struct from specified dungeon data.
//     - name: GetFloorType
//       address:
//         NA: 0x22E03B0
//         EU: 0x22E0CF0
//       description: |-
//         Get the current floor type.
//
//         Floor types:
//           0 appears to mean the current floor is "normal"
//           1 appears to mean the current floor is a fixed floor
//           2 means the current floor has a rescue point
//
//         return: floor type
//     - name: FixedRoomIsSubstituteRoom
//       address:
//         NA: 0x22E08CC
//         EU: 0x22E120C
//       description: |-
//         Checks if the current fixed room is the "substitute room" (ID 0x6E).
//
//         return: bool
//     - name: GenerateDungeonRngSeed
//       address:
//         NA: 0x22EA980
//         EU: 0x22EB330
//       description: |-
//         Generates a seed with which to initialize the dungeon PRNG.
//
//         The seed is calculated by starting with a different seed, the "preseed" x0 (defaults to 1, but can be set by other functions). The preseed is iterated twice with the same recurrence relation used in the primary LCG to generate two pseudorandom 32-bit numbers x1 and x2. The output seed is then computed as
//           seed = (x1 & 0xFF0000) | (x2 >> 0x10) | 1
//         The value x1 is then saved as the new preseed.
//
//         This method of seeding the dungeon PRNG appears to be used only sometimes, depending on certain flags in the data for a given dungeon.
//
//         return: RNG seed
//     - name: GetDungeonRngPreseed
//       address:
//         NA: 0x22EA9CC
//         EU: 0x22EB37C
//       description: |-
//         Gets the current preseed stored in the global dungeon PRNG state. See GenerateDungeonRngSeed for more information.
//
//         return: current dungeon RNG preseed
//     - name: SetDungeonRngPreseed
//       address:
//         NA: 0x22EA9DC
//         EU: 0x22EB38C
//       description: |-
//         Sets the preseed in the global dungeon PRNG state. See GenerateDungeonRngSeed for more information.
//
//         r0: preseed
//     - name: InitDungeonRng
//       address:
//         NA: 0x22EA9EC
//         EU: 0x22EB39C
//       description: |-
//         Initialize (or reinitialize) the dungeon PRNG with a given seed. The primary LCG and the five secondary LCGs are initialized jointly, and with the same seed.
//
//         r0: seed
//     - name: DungeonRand16Bit
//       address:
//         NA: 0x22EAA20
//         EU: 0x22EB3D0
//       description: |-
//         Computes a pseudorandom 16-bit integer using the dungeon PRNG.
//
//         Note that the dungeon PRNG is only used in dungeon mode (as evidenced by these functions being in overlay 29). The game uses another lower-quality PRNG (see arm9.yml) for other needs.
//
//         Random numbers are generated with a linear congruential generator (LCG). The game actually maintains 6 separate sequences that can be used for generation: a primary LCG and 5 secondary LCGs. The generator used depends on parameters set on the global PRNG state.
//
//         All dungeon LCGs have a modulus of 2^32 and a multiplier of 1566083941 (see DUNGEON_PRNG_LCG_MULTIPLIER). The primary LCG uses an increment of 1, while the secondary LCGs use an increment of 2531011 (see DUNGEON_PRNG_LCG_INCREMENT_SECONDARY). So, for example, the primary LCG uses the recurrence relation:
//           x = (1566083941*x_prev + 1) % 2^32
//
//         Since the dungeon LCGs generate 32-bit integers rather than 16-bit, the primary LCG yields 16-bit values by taking the upper 16 bits of the computed 32-bit value. The secondary LCGs yield 16-bit values by taking the lower 16 bits of the computed 32-bit value.
//
//         All of the dungeon LCGs have a hard-coded default seed of 1, but in practice the seed is set with a call to InitDungeonRng during dungeon initialization.
//
//         return: pseudorandom int on the interval [0, 65535]
//     - name: DungeonRandInt
//       address:
//         NA: 0x22EAA98
//         EU: 0x22EB448
//       description: |-
//         Compute a pseudorandom integer under a given maximum value using the dungeon PRNG.
//
//         r0: high
//         return: pseudorandom integer on the interval [0, high - 1]
//     - name: DungeonRandRange
//       address:
//         NA: 0x22EAAC0
//         EU: 0x22EB470
//       description: |-
//         Compute a pseudorandom value between two integers using the dungeon PRNG.
//
//         r0: x
//         r1: y
//         return: pseudorandom integer on the interval [min(x, y), max(x, y) - 1]
//     - name: DungeonRandOutcome
//       address:
//         NA:
//           - 0x22EAB20
//           - 0x22EAB50
//       description: |-
//         Returns the result of a possibly biased coin flip (a Bernoulli random variable) with some success probability p, using the dungeon PRNG.
//
//         r0: success percentage (100*p)
//         return: true with probability p, false with probability (1-p)
//     - name: CalcStatusDuration
//       address:
//         NA: 0x22EAB80
//         EU: 0x22EB530
//       description: |-
//         Seems to calculate the duration of a volatile status on a monster.
//
//         r0: entity pointer
//         r1: pointer to a turn range (an array of two shorts {lower, higher})
//         r2: flag for whether or not to factor in the Self Curer IQ skill and the Natural Cure ability
//         return: number of turns for the status condition
//     - name: DungeonRngUnsetSecondary
//       address:
//         NA: 0x22EAC34
//         EU: 0x22EB5E4
//       description: |-
//         Sets the dungeon PRNG to use the primary LCG for subsequent random number generation, and also resets the secondary LCG index back to 0.
//
//         Similar to DungeonRngSetPrimary, but DungeonRngSetPrimary doesn't modify the secondary LCG index if it was already set to something other than 0.
//
//         No params.
//     - name: DungeonRngSetSecondary
//       address:
//         NA: 0x22EAC4C
//         EU: 0x22EB5FC
//       description: |-
//         Sets the dungeon PRNG to use one of the 5 secondary LCGs for subsequent random number generation.
//
//         r0: secondary LCG index
//     - name: DungeonRngSetPrimary
//       address:
//         NA: 0x22EAC64
//       description: |-
//         Sets the dungeon PRNG to use the primary LCG for subsequent random number generation.
//
//         No params.
//     - name: EntityIsValid
//       address:
//         NA:
//           - 0x22F528C
//           - 0x2321438
//       description: |-
//         Checks if an entity pointer points to a valid entity (not entity type 0, which represents no entity).
//
//         r0: entity pointer
//         return: bool
//     - name: ResetDamageDesc
//       address:
//         NA: 0x22F6E18
//         EU: 0x22F77D0
//       description: |-
//         Seems to zero some damage description struct, which is output by the damage calculation function.
//
//         r0: damage description pointer
//     - name: FloorNumberIsEven
//       address:
//         NA: 0x22F73B4
//         EU: 0x22F7D6C
//       description: |-
//         Checks if the current dungeon floor number is even.
//
//         Has a special check to return false for Labyrinth Cave B10F (the Gabite boss fight).
//
//         return: bool
//     - name: HasLowHealth
//       address:
//         NA: 0x22FB610
//         EU: 0x22FC01C
//       description: |-
//         Checks if the entity passed is a valid monster, and if it's at low health (below 25% rounded down)
//
//         r0: entity pointer
//         return: bool
//     - name: IsSpecialStoryAlly
//       address:
//         NA: 0x22FBAD0
//         EU: 0x22FC4CC
//       description: |-
//         Checks if a monster is a special story ally.
//
//         This is a hard-coded check that looks at the monster's "Joined At" field. If the value is in the range [DUNGEON_JOINED_AT_BIDOOF, DUNGEON_DUMMY_0xE3], this check will return true.
//
//         r0: monster pointer
//         return: bool
//     - name: IsExperienceLocked
//       address:
//         NA: 0x22FBAF0
//         EU: 0x22FC4EC
//       description: |-
//         Checks if a monster does not gain experience.
//
//         This basically just inverts IsSpecialStoryAlly, with the exception of also checking for the "Joined At" field being DUNGEON_CLIENT (is this set for mission clients?).
//
//         r0: monster pointer
//         return: bool
//     - name: ItemIsActive
//       address:
//         NA:
//           - 0x22FF898
//           - 0x23026CC
//           - 0x230A9DC
//           - 0x2311034
//         EU: 0x23002C4
//       description: |-
//         Checks if a monster is holding a certain item that isn't disabled by Klutz.
//
//         r0: entity pointer
//         r1: item ID
//         return: bool
//     - name: SprintfStatic
//       address:
//         NA: 0x23002C8
//         EU: 0x2300CF4
//       description: |-
//         Statically defined copy of sprintf(3) in overlay 29. See arm9.yml for more information.
//
//         r0: str
//         r1: format
//         ...: variadic
//         return: number of characters printed, excluding the null-terminator
//     - name: IsMonster
//       address:
//         NA:
//           - 0x2301A60
//           - 0x230A994
//       description: |-
//         Checks if an entity is a monster (entity type 1).
//
//         r0: entity pointer
//         return: bool
//     - name: NoGastroAcidStatus
//       address:
//         NA: 0x2301CDC
//         EU: 0x2302708
//       description: |-
//         Checks if a monster does not have the Gastro Acid status.
//
//         r0: entity pointer
//         return: bool
//     - name: AbilityIsActive
//       address:
//         NA: 0x2301D10
//         EU: 0x230273C
//       description: |-
//         Checks if a monster has a certain ability that isn't disabled by Gastro Acid.
//
//         r0: entity pointer
//         r1: ability ID
//         return: bool
//     - name: MonsterIsType
//       address:
//         NA: 0x2301E50
//         EU: 0x230287C
//       description: |-
//         Checks if a monster is a given type.
//
//         r0: entity pointer
//         r1: type ID
//         return: bool
//     - name: IqSkillIsEnabled
//       address:
//         NA: 0x2301F80
//         EU: 0x23029AC
//       description: |-
//         Checks if a monster has a certain IQ skill enabled.
//
//         r0: entity pointer
//         r1: IQ skill ID
//         return: bool
//     - name: GetMovePower
//       address:
//         NA: 0x230231C
//         EU: 0x2302D48
//       description: |-
//         Gets the power of a move, factoring in Ginseng/Space Globe boosts.
//
//         r0: user pointer
//         r1: move pointer
//         return: move power
//     - name: AddExpSpecial
//       address:
//         NA: 0x230253C
//         EU: 0x2302F68
//       description: |-
//         Adds to a monster's experience points, subject to experience boosting effects.
//
//         This function appears to be called only under special circumstances. Possibly when granting experience from damage (e.g., Joy Ribbon)?
//
//         Interestingly, the parameter in r0 isn't actually used. This might be a compiler optimization to avoid shuffling registers, since this function might be called alongside lots of other functions that have both the attacker and defender as the first two arguments.
//
//         r0: attacker pointer
//         r1: defender pointer
//         r2: base experience gain, before boosts
//     - name: PptrIsValid
//       address:
//         NA:
//           - 0x2308FBC
//           - 0x23118B4
//           - 0x2315118
//           - 0x232800C
//       description: |-
//         Checks if a double pointer is non-null when double-dereferenced.
//
//         r0: pointer to pointer
//         return: bool
//     - name: DefenderAbilityIsActive
//       address:
//         NA:
//           - 0x230A940
//           - 0x2311B94
//       description: |-
//         Checks if a defender has an active ability that isn't disabled by an attacker's Mold Breaker.
//
//         r0: attacker pointer
//         r1: defender pointer
//         r2: ability ID to check on the defender
//         r3: flag for whether the attacker's ability is enabled
//         return: bool
//     - name: ExclusiveItemEffectIsActive
//       address:
//         NA: 0x230A9B8
//         EU: 0x230B42C
//       description: |-
//         Checks if a certain exclusive item effect is active for a monster.
//
//         r0: entity pointer
//         r1: exclusive item effect ID
//         return: bool
//     - name: GetTypeMatchup
//       address:
//         NA: 0x230AC58
//         EU: 0x230B6CC
//       description: |-
//         Gets the type matchup for a given combat interaction.
//
//         Note that the actual monster's types on the attacker and defender pointers are not used; the pointers are only used to check conditions. The actual type matchup table lookup is done solely using the attack and target type parameters.
//
//         This factors in some conditional effects like exclusive items, statuses, etc. There's some weirdness with the Ghost type; see the comment for struct type_matchup_table.
//
//         r0: attacker pointer
//         r1: defender pointer
//         r2: target type index (0 the target's first type, 1 for the target's second type)
//         r3: attack type
//         return: enum type_matchup
//     - name: CalcDamage
//       address:
//         NA: 0x230BBAC
//         EU: 0x230C620
//       description: |-
//         Probably the damage calculation function.
//
//         r0: attacker pointer
//         r1: defender pointer
//         r2: attack type
//         r3: attack power
//         stack[0]: crit chance
//         stack[1]: [output] damage description struct?
//         stack[2]: damage multiplier (as a Q24.8 fixed-point float)
//         stack[3]: move ID
//         stack[4]: ?
//     - name: CalcRecoilDamageFixed
//       address:
//         NA: 0x230D18C
//         EU: 0x230DC00
//       description: |-
//         Appears to calculate recoil damage to a monster.
//
//         This function wraps CalcDamageFixed using the monster as both the attacker and the defender, after doing some basic checks (like if the monster is already at 0 HP) and applying a boost from the Reckless ability if applicable.
//
//         r0: entity pointer
//         r1: fixed damage
//         r2: ?
//         r3: [output] damage description struct?
//         stack[0]: move ID (interestingly, this doesn't seem to be used by the function)
//         stack[1]: attack type
//         stack[2]: ?
//         stack[3]: message type
//         others: ?
//     - name: CalcDamageFixed
//       address:
//         NA: 0x230D240
//         EU: 0x230DCB4
//       description: |-
//         Appears to calculate damage from a fixed-damage effect.
//
//         r0: attacker pointer
//         r1: defender pointer
//         r2: fixed damage
//         r3: ?
//         stack[0]: [output] damage description struct?
//         stack[1]: attack type
//         stack[2]: move category
//         stack[3]: ?
//         stack[4]: message type
//         others: ?
//     - name: CalcDamageFixedNoCategory
//       address:
//         NA: 0x230D3A8
//         EU: 0x230DE1C
//       description: |-
//         A wrapper around CalcDamageFixed with the move category set to none.
//
//         r0: attacker pointer
//         r1: defender pointer
//         r2: fixed damage
//         stack[0]: [output] damage description struct?
//         stack[1]: attack type
//         others: ?
//     - name: CalcDamageFixedWrapper
//       address:
//         NA: 0x230D3F4
//         EU: 0x230DE68
//       description: |-
//         A wrapper around CalcDamageFixed.
//
//         r0: attacker pointer
//         r1: defender pointer
//         r2: fixed damage
//         stack[0]: [output] damage description struct?
//         stack[1]: attack type
//         stack[2]: move category
//         others: ?
//     - name: ResetDamageCalcScratchSpace
//       address:
//         NA: 0x230D528
//         EU: 0x230DF9C
//       description: |-
//         CalcDamage seems to use scratch space of some kind, which this function zeroes.
//
//         No params.
//     - name: AuraBowIsActive
//       address:
//         NA: 0x230F6C8
//         EU: 0x231013C
//       description: |-
//         Checks if a monster is holding an aura bow that isn't disabled by Klutz.
//
//         r0: entity pointer
//         return: bool
//     - name: ExclusiveItemOffenseBoost
//       address:
//         NA: 0x230F778
//         EU: 0x23101EC
//       description: |-
//         Gets the exclusive item boost for attack/special attack for a monster
//
//         r0: entity pointer
//         r1: move category index (0 for physical, 1 for special)
//         return: boost
//     - name: ExclusiveItemDefenseBoost
//       address:
//         NA: 0x230F788
//         EU: 0x23101FC
//       description: |-
//         Gets the exclusive item boost for defense/special defense for a monster
//
//         r0: entity pointer
//         r1: move category index (0 for physical, 1 for special)
//         return: boost
//     - name: InflictSleepStatusSingle
//       address:
//         NA: 0x2311824
//         EU: 0x2312284
//       description: |-
//         This is called by TryInflictSleepStatus.
//
//         r0: entity pointer
//         r1: number of turns
//     - name: TryInflictSleepStatus
//       address:
//         NA: 0x23118D8
//         EU: 0x2312338
//       description: |-
//         Inflicts the Sleep status condition on a target monster if possible.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: number of turns
//         r3: flag to log a message on failure
//     - name: TryInflictNightmareStatus
//       address:
//         NA: 0x2311C4C
//         EU: 0x23126AC
//       description: |-
//         Inflicts the Nightmare status condition on a target monster if possible.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: number of turns
//     - name: TryInflictNappingStatus
//       address:
//         NA: 0x2311D60
//         EU: 0x23127C0
//       description: |-
//         Inflicts the Napping status condition (from Rest) on a target monster if possible.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: number of turns
//     - name: TryInflictYawningStatus
//       address:
//         NA: 0x2311E70
//         EU: 0x23128D0
//       description: |-
//         Inflicts the Yawning status condition on a target monster if possible.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: number of turns
//     - name: TryInflictSleeplessStatus
//       address:
//         NA: 0x2311F80
//         EU: 0x23129E0
//       description: |-
//         Inflicts the Sleepless status condition on a target monster if possible.
//
//         r0: user entity pointer
//         r1: target entity pointer
//     - name: TryInflictPausedStatus
//       address:
//         NA: 0x231206C
//         EU: 0x2312ACC
//       description: |-
//         Inflicts the Paused status condition on a target monster if possible.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: ?
//         r3: number of turns
//         stack[0]: flag to log a message on failure
//         stack[1]: flag to only perform the check for inflicting without actually inflicting
//         return: Whether or not the status could be inflicted
//     - name: TryInflictInfatuatedStatus
//       address:
//         NA: 0x23121AC
//         EU: 0x2312C0C
//       description: |-
//         Inflicts the Infatuated status condition on a target monster if possible.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: flag to log a message on failure
//         r3: flag to only perform the check for inflicting without actually inflicting
//         return: Whether or not the status could be inflicted
//     - name: TryInflictBurnStatus
//       address:
//         NA: 0x2312338
//         EU: 0x2312D98
//       description: |-
//         Inflicts the Burn status condition on a target monster if possible.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: flag to apply some special effect alongside the burn?
//         r3: flag to log a message on failure
//         stack[0]: flag to only perform the check for inflicting without actually inflicting
//         return: Whether or not the status could be inflicted
//     - name: TryInflictBurnStatusWholeTeam
//       address:
//         NA: 0x2312618
//         EU: 0x2313078
//       description: |-
//         Inflicts the Burn status condition on all team members if possible.
//
//         No params.
//     - name: TryInflictPoisonedStatus
//       address:
//         NA: 0x2312664
//         EU: 0x23130C4
//       description: |-
//         Inflicts the Poisoned status condition on a target monster if possible.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: flag to log a message on failure
//         r3: flag to only perform the check for inflicting without actually inflicting
//         return: Whether or not the status could be inflicted
//     - name: TryInflictBadlyPoisonedStatus
//       address:
//         NA: 0x231293C
//         EU: 0x231339C
//       description: |-
//         Inflicts the Badly Poisoned status condition on a target monster if possible.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: flag to log a message on failure
//         r3: flag to only perform the check for inflicting without actually inflicting
//         return: Whether or not the status could be inflicted
//     - name: TryInflictFrozenStatus
//       address:
//         NA: 0x2312BF8
//         EU: 0x2313658
//       description: |-
//         Inflicts the Frozen status condition on a target monster if possible.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: flag to log a message on failure
//     - name: TryInflictConstrictionStatus
//       address:
//         NA: 0x2312E20
//         EU: 0x2313880
//       description: |-
//         Inflicts the Constriction status condition on a target monster if possible.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: animation ID
//         r3: flag to log a message on failure
//     - name: TryInflictShadowHoldStatus
//       address:
//         NA: 0x2312F78
//         EU: 0x23139D8
//       description: |-
//         Inflicts the Shadow Hold (AKA Immobilized) status condition on a target monster if possible.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: flag to log a message on failure
//     - name: TryInflictIngrainStatus
//       address:
//         NA: 0x2313130
//         EU: 0x2313B90
//       description: |-
//         Inflicts the Ingrain status condition on a target monster if possible.
//
//         r0: user entity pointer
//         r1: target entity pointer
//     - name: TryInflictWrappedStatus
//       address:
//         NA: 0x23131F4
//         EU: 0x2313C54
//       description: |-
//         Inflicts the Wrapped status condition on a target monster if possible.
//
//         This also gives the user the Wrap status (Wrapped around foe).
//
//         r0: user entity pointer
//         r1: target entity pointer
//     - name: TryInflictPetrifiedStatus
//       address:
//         NA: 0x231346C
//         EU: 0x2313ECC
//       description: |-
//         Inflicts the Petrified status condition on a target monster if possible.
//
//         r0: user entity pointer
//         r1: target entity pointer
//     - name: LowerOffensiveStat
//       address:
//         NA: 0x23135FC
//         EU: 0x231405C
//       description: |-
//         Lowers the specified offensive stat on the target monster.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: stat index
//         r3: number of stages
//         stack[0]: ?
//         stack[1]: ?
//     - name: LowerDefensiveStat
//       address:
//         NA: 0x2313814
//         EU: 0x2314274
//       description: |-
//         Lowers the specified defensive stat on the target monster.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: stat index
//         r3: number of stages
//         stack[0]: ?
//         stack[1]: ?
//     - name: BoostOffensiveStat
//       address:
//         NA: 0x231399C
//         EU: 0x23143FC
//       description: |-
//         Boosts the specified offensive stat on the target monster.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: stat index
//         r3: number of stages
//     - name: BoostDefensiveStat
//       address:
//         NA: 0x2313B08
//         EU: 0x2314568
//       description: |-
//         Boosts the specified defensive stat on the target monster.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: stat index
//         r3: number of stages
//     - name: ApplyOffensiveStatMultiplier
//       address:
//         NA: 0x2313D40
//         EU: 0x23147A0
//       description: |-
//         Applies a multiplier to the specified offensive stat on the target monster.
//
//         This affects struct monster_stat_modifiers::offensive_multipliers, for moves like Charm and Memento.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: stat index
//         r3: multiplier
//         stack[0]: ?
//     - name: ApplyDefensiveStatMultiplier
//       address:
//         NA: 0x2313F64
//         EU: 0x23149C4
//       description: |-
//         Applies a multiplier to the specified defensive stat on the target monster.
//
//         This affects struct monster_stat_modifiers::defensive_multipliers, for moves like Screech.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: stat index
//         r3: multiplier
//         stack[0]: ?
//     - name: BoostHitChanceStat
//       address:
//         NA: 0x23140E4
//         EU: 0x2314B44
//       description: |-
//         Boosts the specified hit chance stat (accuracy or evasion) on the target monster.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: stat index
//     - name: LowerHitChanceStat
//       address:
//         NA: 0x231422C
//         EU: 0x2314C8C
//       description: |-
//         Lowers the specified hit chance stat (accuracy or evasion) on the target monster.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: stat index
//         r3: ?
//     - name: TryInflictCringeStatus
//       address:
//         NA: 0x23143E8
//         EU: 0x2314E48
//       description: |-
//         Inflicts the Cringe status condition on a target monster if possible.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: flag to log a message on failure
//         r3: flag to only perform the check for inflicting without actually inflicting
//         return: Whether or not the status could be inflicted
//     - name: TryInflictParalysisStatus
//       address:
//         NA: 0x2314544
//         EU: 0x2314FA4
//       description: |-
//         Inflicts the Paralysis status condition on a target monster if possible.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: flag to log a message on failure
//         r3: flag to only perform the check for inflicting without actually inflicting
//         return: Whether or not the status could be inflicted
//     - name: TeamExclusiveItemEffectIsActive
//       address:
//         NA:
//           - 0x23147EC
//           - 0x231A87C
//         EU: 0x231524C
//       description: |-
//         Checks if a monster is a team member under the effects of a certain exclusive item effect.
//
//         r0: entity pointer
//         r1: exclusive item effect ID
//         return: bool
//     - name: BoostSpeed
//       address:
//         NA: 0x2314810
//         EU: 0x2315270
//       description: |-
//         Boosts the speed of the target monster.
//
//         If the number of turns specified is 0, a random turn count will be selected using the default SPEED_BOOST_DURATION_RANGE.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: number of stages
//         r3: number of turns
//         stack[0]: flag to log a message on failure
//     - name: BoostSpeedOneStage
//       address:
//         NA: 0x231493C
//         EU: 0x231539C
//       description: |-
//         A wrapper around BoostSpeed with the number of stages set to 1.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: number of turns
//         r3: flag to log a message on failure
//     - name: LowerSpeed
//       address:
//         NA: 0x2314954
//         EU: 0x23153B4
//       description: |-
//         Lowers the speed of the target monster.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: number of stages
//         r3: flag to log a message on failure
//     - name: TrySealMove
//       address:
//         NA: 0x2314ABC
//         EU: 0x231551C
//       description: |-
//         Seals one of the target monster's moves. The move to be sealed is randomly selected.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: flag to log a message on failure
//         return: Whether or not a move was sealed
//     - name: BoostOrLowerSpeed
//       address:
//         NA: 0x2314C2C
//         EU: 0x231568C
//       description: |-
//         Randomly boosts or lowers the speed of the target monster by one stage with equal probability.
//
//         r0: user entity pointer
//         r1: target entity pointer
//     - name: ResetHitChanceStat
//       address:
//         NA: 0x2314C8C
//         EU: 0x23156EC
//       description: |-
//         Resets the specified hit chance stat (accuracy or evasion) back to normal on the target monster.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: stat index
//         r3: ?
//     - name: TryActivateQuickFeet
//       address:
//         NA: 0x2314E1C
//         EU: 0x231587C
//       description: |-
//         Activate the Quick Feet ability on the defender, if the monster has it and it's active.
//
//         r0: attacker pointer
//         r1: defender pointer
//         return: bool, whether or not the ability was activated
//     - name: TryInflictConfusedStatus
//       address:
//         NA: 0x2314F38
//         EU: 0x2315998
//       description: |-
//         Inflicts the Confused status condition on a target monster if possible.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: flag to log a message on failure
//         r3: flag to only perform the check for inflicting without actually inflicting
//         return: Whether or not the status could be inflicted
//     - name: TryInflictCoweringStatus
//       address:
//         NA: 0x231516C
//         EU: 0x2315CCC
//       description: |-
//         Inflicts the Cowering status condition on a target monster if possible.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: flag to log a message on failure
//         r3: flag to only perform the check for inflicting without actually inflicting
//         return: Whether or not the status could be inflicted
//     - name: TryIncreaseHp
//       address:
//         NA: 0x23152E4
//         EU: 0x2315D44
//       description: |-
//         Restore HP and possibly boost max HP of the target monster if possible.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: HP to restore
//         r3: max HP boost
//         stack[0]: flag to log a message on failure
//         return: Success flag
//     - name: TryInflictLeechSeedStatus
//       address:
//         NA: 0x23157EC
//         EU: 0x231624C
//       description: |-
//         Inflicts the Leech Seed status condition on a target monster if possible.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: flag to log a message on failure
//         r3: flag to only perform the check for inflicting without actually inflicting
//         return: Whether or not the status could be inflicted
//     - name: TryInflictDestinyBond
//       address:
//         NA: 0x2315A50
//         EU: 0x23164B0
//       description: |-
//         Inflicts the Destiny Bond status condition on a target monster if possible.
//
//         r0: user entity pointer
//         r1: target entity pointer
//     - name: RestoreMovePP
//       address:
//         NA: 0x2317C20
//         EU: 0x2318680
//       description: |-
//         Restores the PP of all the target's moves by the specified amount.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: PP to restore
//         r3: flag to suppress message logging
//     - name: HasConditionalGroundImmunity
//       address:
//         NA: 0x2318A4C
//         EU: 0x23194AC
//       description: |-
//         Checks if a monster is currently immune to Ground-type moves for reasons other than typing and ability.
//
//         This includes checks for Gravity and Magnet Rise.
//
//         r0: entity pointer
//         return: bool
//     - name: GetEntityMoveTargetAndRange
//       address:
//         NA: 0x231ACAC
//         EU: 0x231B70C
//       description: |-
//         Gets the move target-and-range field when used by a given entity. See struct move_target_and_range in the C headers.
//
//         r0: entity pointer
//         r1: move pointer
//         r2: AI flag (same as GetMoveTargetAndRange)
//         return: move target and range
//     - name: ApplyItemEffect
//       address:
//         NA: 0x231B68C
//         EU: 0x231C0EC
//       description: |-
//         Seems to apply an item's effect via a giant switch statement?
//
//         r3: attacker pointer
//         stack[0]: defender pointer
//         stack[1]: thrown item pointer
//         others: ?
//     - name: ViolentSeedBoost
//       address:
//         NA: 0x231CE1C
//         EU: 0x231D884
//       description: |-
//         Applies the Violent Seed boost to an entity.
//
//         r0: attacker pointer
//         r1: defender pointer
//     - name: ApplyGummiBoosts
//       address:
//         NA: 0x231D0C0
//         EU: 0x231DB28
//       description: |-
//         Applies the IQ and possible stat boosts from eating a Gummi to the target monster.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: Gummi type ID
//         r3: Stat boost amount, if a random stat boost occurs
//     - name: GetMaxPpWrapper
//       address:
//         NA: 0x231E9F0
//         EU: 0x231F458
//       description: |-
//         Gets the maximum PP for a given move. A wrapper around the function in the ARM 9 binary.
//
//         r0: move pointer
//         return: max PP for the given move, capped at 99
//     - name: MoveIsNotPhysical
//       address:
//         NA: 0x231EA18
//         EU: 0x231F480
//       description: |-
//         Checks if a move isn't a physical move.
//
//         r0: move ID
//         return: bool
//     - name: TryPounce
//       address:
//         NA: 0x231FC20
//         EU: 0x2320688
//       description: |-
//         Makes the target monster execute the Pounce action in a given direction if possible.
//
//         If the direction ID is 8, the target will pounce in the direction it's currently facing.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: direction ID
//     - name: TryBlowAway
//       address:
//         NA: 0x231FDE0
//         EU: 0x2320848
//       description: |-
//         Blows away the target monster in a given direction if possible.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: direction ID
//     - name: TryWarp
//       address:
//         NA: 0x2320D08
//         EU: 0x2321770
//       description: |-
//         Makes the target monster warp if possible.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: warp type
//         r3: position (if warp type is position-based)
//     - name: DungeonRandOutcomeUserTargetInteraction
//       address:
//         NA: 0x2324934
//         EU: 0x232539C
//       description: |-
//         Like DungeonRandOutcome, but specifically for user-target interactions.
//
//         This modifies the underlying random process depending on factors like Serene Grace, and whether or not either entity has fainted.
//
//         r0: user entity pointer
//         r1: target entity pointer
//         r2: base success percentage (100*p). 0 is treated specially and guarantees success.
//     - name: DungeonRandOutcomeUserAction
//       address:
//         NA: 0x2324A20
//         EU: 0x2325488
//       description: |-
//         Like DungeonRandOutcome, but specifically for user actions.
//
//         This modifies the underlying random process to factor in Serene Grace (and checks whether the user is a valid entity).
//
//         r0: entity pointer
//         r1: base success percentage (100*p). 0 is treated specially and guarantees success.
//     - name: UpdateMovePp
//       address:
//         NA: 0x2324D8C
//         EU: 0x23257F4
//       description: |-
//         Updates the PP of any moves that were used by a monster, if PP should be consumed.
//
//         r0: entity pointer
//         r1: flag for whether or not PP should be consumed
//     - name: LowerSshort
//       address:
//         NA: 0x2324E64
//         EU: 0x23258CC
//       description: |-
//         Gets the lower 2 bytes of a 4-byte number and interprets it as a signed short.
//
//         r0: 4-byte number x
//         return: (short)x
//     - name: DealDamageWithRecoil
//       address:
//         NA: 0x2327F34
//         EU: 0x23289A0
//       description: |-
//         Deals damage from a move or item used by an attacking monster on a defending monster, and also deals recoil damage to the attacker.
//
//         r0: attacker pointer
//         r1: defender pointer
//         r2: move
//         r3: item ID
//         return: bool, whether or not damage was dealt
//     - name: DealDamage
//       address:
//         NA: 0x2332B20
//         EU: 0x2333560
//       description: |-
//         Deals damage from a move or item used by an attacking monster on a defending monster.
//
//         r0: attacker pointer
//         r1: defender pointer
//         r2: move
//         r3: damage multiplier (as a Q24.8 fixed-point float)
//         stack[0]: item ID
//         return: amount of damage dealt
//     - name: CalcDamageProjectile
//       address:
//         NA: 0x2332C4C
//         EU: 0x233368C
//       description: |-
//         Appears to calculate damage from a variable-damage projectile.
//
//         r0: entity pointer 1?
//         r1: entity pointer 2?
//         r2: move pointer
//         r3: move power
//         others: ?
//     - name: GetApparentWeather
//       address:
//         NA: 0x2334D08
//         EU: 0x2335748
//       description: |-
//         Get the weather, as experienced by a specific entity.
//
//         r0: entity pointer
//         return: weather ID
//     - name: GetTile
//       address:
//         NA:
//           - 0x23360FC
//           - 0x2336164
//       description: |-
//         Get the tile at some position.
//
//         r0: x position
//         r1: y position
//         return: tile pointer
//     - name: GravityIsActive
//       address:
//         NA: 0x2338390
//         EU: 0x2338F60
//       description: |-
//         Checks if gravity is active on the floor.
//
//         return: bool
//     - name: IsSecretBazaar
//       address:
//         NA: 0x23385C4
//         EU: 0x2339194
//       description: |-
//         Checks if the current floor is the Secret Bazaar.
//
//         return: bool
//     - name: IsSecretRoom
//       address:
//         NA: 0x233865C
//         EU: 0x233922C
//       description: |-
//         Checks if the current floor is the Secret Room fixed floor (from hidden stairs).
//
//         return: bool
//     - name: LoadFixedRoomDataVeneer
//       address:
//         NA: 0x233A624
//       description: |-
//         Likely a linker-generated veneer for LoadFixedRoomData.
//
//         See https://developer.arm.com/documentation/dui0474/k/image-structure-and-generation/linker-generated-veneers/what-is-a-veneer-
//
//         No params.
//     - name: IsNormalFloor
//       address:
//         NA: 0x233A654
//         EU: 0x233B238
//       description: |-
//         Checks if the current floor is a normal layout.
//
//         "Normal" means any layout that is NOT one of the following:
//         - Hidden stairs floors
//         - Golden Chamber
//         - Challenge Request floor
//         - Outlaw hideout
//         - Treasure Memo floor
//         - Full-room fixed floors (ID < 0xA5) [0xA5 == Sealed Chamber]
//
//         return: bool
//     - name: GenerateFloor
//       address:
//         NA: 0x233A6D8
//         EU: 0x233B2BC
//       description: |-
//         This is the master function that generates the dungeon floor.
//
//         Very loosely speaking, this function first tries to generate a valid floor layout. Then it tries to spawn entities in a valid configuration. Finally, it performs cleanup and post-processing depending on the dungeon.
//
//         If a spawn configuration is invalid, the entire floor layout is scrapped and regenerated. If the generated floor layout is invalid 10 times in a row, or a valid spawn configuration isn't generated within 10 attempts, the generation algorithm aborts and the default one-room Monster House floor is generated as a fallback.
//
//         No params.
//     - name: GetTileTerrain
//       address:
//         NA: 0x233AE78
//         EU: 0x233BA5C
//       description: |-
//         Gets the terrain type of a tile.
//
//         r0: tile pointer
//         return: terrain ID
//     - name: DungeonRand100
//       address:
//         NA: 0x233AE84
//         EU: 0x233BA68
//       description: |-
//         Compute a pseudorandom integer on the interval [0, 100) using the dungeon PRNG.
//
//         return: pseudorandom integer
//     - name: FlagHallwayJunctions
//       address:
//         NA: 0x233AF0C
//         EU: 0x233BAF0
//       description: |-
//         Sets the junction flag (bit 3 of the terrain flags) on any hallway junction tiles in some range [x0, x1), [y0, y1). This leaves tiles within rooms untouched.
//
//         A hallway tile is considered a junction if it has at least 3 cardinal neighbors with open terrain.
//
//         r0: x0
//         r1: y0
//         r2: x1
//         r3: y1
//     - name: GenerateStandardFloor
//       address:
//         NA: 0x233B028
//         EU: 0x233BC0C
//       description: |-
//         Generate a standard floor with the given parameters.
//
//         Broadly speaking, a standard floor is generated as follows:
//         1. Generating the grid
//         2. Creating a room or hallway anchor in each grid cell
//         3. Creating hallways between grid cells
//         4. Generating special features (maze room, Kecleon shop, Monster House, extra hallways, room imperfections, secondary structures)
//
//         r0: grid size x
//         r1: grid size y
//         r2: floor properties
//     - name: GenerateOuterRingFloor
//       address:
//         NA: 0x233B190
//         EU: 0x233BD74
//       description: |-
//         Generates a floor layout with a 4x2 grid of rooms, surrounded by an outer ring of hallways.
//
//         r0: floor properties
//     - name: GenerateCrossroadsFloor
//       address:
//         NA: 0x233B61C
//         EU: 0x233C200
//       description: |-
//         Generates a floor layout with a mesh of hallways on the interior 3x2 grid, surrounded by a boundary of rooms protruding from the interior like spikes, excluding the corner cells.
//
//         r0: floor properties
//     - name: GenerateLineFloor
//       address:
//         NA: 0x233BA7C
//         EU: 0x233C660
//       description: |-
//         Generates a floor layout with 5 grid cells in a horizontal line.
//
//         r0: floor properties
//     - name: GenerateCrossFloor
//       address:
//         NA: 0x233BBDC
//         EU: 0x233C7C0
//       description: |-
//         Generates a floor layout with 5 rooms arranged in a cross ("plus sign") formation.
//
//         r0: floor properties
//     - name: GenerateBeetleFloor
//       address:
//         NA: 0x233BD74
//         EU: 0x233C958
//       description: |-
//         Generates a floor layout in a "beetle" formation, which is created by taking a 3x3 grid of rooms, connecting the rooms within each row, and merging the central column into one big room.
//
//         r0: floor properties
//     - name: MergeRoomsVertically
//       address:
//         NA: 0x233BF30
//         EU: 0x233CB14
//       description: |-
//         Merges two vertically stacked rooms into one larger room.
//
//         r0: x grid coordinate of the rooms to merge
//         r1: y grid coordinate of the upper room
//         r2: dy, where the lower room has a y grid coordinate of y+dy
//         r3: grid to update
//     - name: GenerateOuterRoomsFloor
//       address:
//         NA: 0x233C07C
//         EU: 0x233CC60
//       description: |-
//         Generates a floor layout with a ring of rooms on the grid boundary and nothing in the interior.
//
//         Note that this function is bugged, and won't properly connect all the rooms together for grid_size_x < 4.
//
//         r0: grid size x
//         r1: grid size y
//         r2: floor properties
//     - name: IsNotFullFloorFixedRoom
//       address:
//         NA: 0x233C310
//         EU: 0x233CEF4
//       description: |-
//         Checks if a fixed room ID does not correspond to a fixed, full-floor layout.
//
//         The first non-full-floor fixed room is 0xA5, which is for Sealed Chambers.
//
//         r0: fixed room ID
//         return: bool
//     - name: GenerateFixedRoom
//       address:
//         NA: 0x233C32C
//         EU: 0x233CF10
//       description: |-
//         Handles fixed room generation if the floor contains a fixed room.
//
//         r0: fixed room ID
//         r1: floor properties
//         return: bool
//     - name: GenerateOneRoomMonsterHouseFloor
//       address:
//         NA: 0x233C774
//         EU: 0x233D358
//       description: |-
//         Generates a floor layout with just a large, one-room Monster House.
//
//         This is the default layout if dungeon generation fails.
//
//         No params.
//     - name: GenerateTwoRoomsWithMonsterHouseFloor
//       address:
//         NA: 0x233C844
//         EU: 0x233D428
//       description: |-
//         Generate a floor layout with two rooms (left and right), one of which is a Monster House.
//
//         No params.
//     - name: GenerateExtraHallways
//       address:
//         NA: 0x233C9E8
//         EU: 0x233D5CC
//       description: |-
//         Generate extra hallways on the floor via a series of random walks.
//
//         Each random walk starts from a random tile in a random room, leaves the room in a random cardinal direction, and from there tunnels through obstacles through a series of random turns, leaving open terrain in its wake. The random walk stops when it reaches open terrain, goes out of bounds, or reaches an impassable obstruction.
//
//         r0: grid to update
//         r1: grid size x
//         r2: grid size y
//         r3: number of extra hallways to generate
//     - name: GetGridPositions
//       address:
//         NA: 0x233CF84
//         EU: 0x233DB68
//       description: |-
//         Get the grid cell positions for a given set of floor grid dimensions.
//
//         r0: [output] pointer to array of the starting x coordinates of each grid column
//         r1: [output] pointer to array of the starting y coordinates of each grid row
//         r2: grid size x
//         r3: grid size y
//     - name: InitDungeonGrid
//       address:
//         NA: 0x233D004
//         EU: 0x233DBE8
//       description: |-
//         Initialize a dungeon grid with defaults.
//
//         The grid is an array of grid cells stored in column-major order (such that grid cells with the same x value are stored contiguously), with a fixed column size of 15. If the grid size in the y direction is less than this, the last (15 - grid_size_y) entries of each column will be uninitialized.
//
//         Note that the grid size arguments define the maximum size of the grid from a programmatic standpoint. However, grid cells can be invalidated if they exceed the configured floor size in the dungeon generation status struct. Thus, the dimensions of the ACTIVE grid can be smaller.
//
//         r0: [output] grid (expected to have space for at least (15*(grid_size_x-1) + grid_size_y) dungeon grid cells)
//         r1: grid size x
//         r2: grid size y
//     - name: AssignRooms
//       address:
//         NA: 0x233D104
//         EU: 0x233DCE8
//       description: |-
//         Randomly selects a subset of grid cells to become rooms.
//
//         The given number of grid cells will become rooms. If any of the selected grid cells are invalid, fewer rooms will be generated. The number of rooms assigned will always be at least 2 and never exceed 36.
//
//         Cells not marked as rooms will become hallway anchors. A hallway anchor is a single tile in a non-room grid cell to which hallways will be connected later, thus "anchoring" hallway generation.
//
//         r0: grid to update
//         r1: grid size x
//         r2: grid size y
//         r3: number of rooms; if positive, a random value between [n_rooms, n_rooms+2] will be used. If negative, |n_rooms| will be used exactly.
//     - name: CreateRoomsAndAnchors
//       address:
//         NA: 0x233D318
//         EU: 0x233DEFC
//       description: |-
//         Creates rooms and hallway anchors in each grid cell as designated by AssignRooms.
//
//         This function creates a rectangle of open terrain for each room (with some margin relative to the grid cell border). A single open tile is created in hallway anchor cells, and a hallway anchor indicator is set for later reference.
//
//         r0: grid to update
//         r1: grid size x
//         r2: grid size y
//         r3: array of the starting x coordinates of each grid column
//         stack[0]: array of the starting y coordinates of each grid row
//         stack[1]: room bitflags; only uses bit 2 (mask: 0b100), which enables room imperfections
//     - name: GenerateSecondaryStructures
//       address:
//         NA: 0x233D674
//         EU: 0x233E258
//       description: |-
//         Try to generate secondary structures in flagged rooms.
//
//         If a valid room with no special features is flagged to have a secondary structure, try to generate a random one in the room, based on the result of a dice roll:
//           0: no secondary structure
//           1: maze, or a central water/lava "plus sign" as fallback, or a single water/lava tile in the center as a second fallback
//           2: checkerboard pattern of water/lava
//           3: central pool of water/lava
//           4: central "island" with items and a Warp Tile, surrounded by a "moat" of water/lava
//           5: horizontal or vertical divider of water/lava splitting the room in two
//
//         If the room isn't the right shape, dimension, or otherwise doesn't support the selected secondary structure, it is left untouched.
//
//         r0: grid to update
//         r1: grid size x
//         r2: grid size y
//     - name: AssignGridCellConnections
//       address:
//         NA: 0x233E05C
//         EU: 0x233EC40
//       description: |-
//         Randomly assigns connections between adjacent grid cells.
//
//         Connections are created via a random walk with momentum, starting from the grid cell at (cursor x, cursor y). A connection is drawn in a random direction from the current cursor, and this process is repeated a certain number of times (the "floor connectivity" specified in the floor properties). The direction of the random walk has "momentum"; there's a 50% chance it will be the same as the previous step (or rotated counterclockwise if on the boundary). This helps to reduce the number of dead ends and forks in the road caused by the random walk "doubling back" on itself.
//
//         If dead ends are disabled in the floor properties, there is an additional phase to remove dead end hallway anchors (only hallway anchors, not rooms) by drawing additional connections. Note that the actual implementation contains a bug: the grid cell validity checks use the wrong index, so connections may be drawn to invalid cells.
//
//         r0: grid to update
//         r1: grid size x
//         r2: grid size y
//         r3: cursor x
//         stack[0]: cursor y
//         stack[1]: floor properties
//     - name: CreateGridCellConnections
//       address:
//         NA: 0x233E43C
//         EU: 0x233F020
//       description: |-
//         Create grid cell connections either by creating hallways or merging rooms.
//
//         When creating a hallway connecting a hallway anchor, the exact anchor coordinates are used as the endpoint. When creating a hallway connecting a room, a random point on the room edge facing the hallway is used as the endpoint. The grid cell boundaries are used as the middle coordinates for kinks (see CreateHallway).
//
//         If room merging is enabled, there is a 9.75% chance that two connected rooms will be merged into a single larger room (9.75% comes from two 5% rolls, one for each of the two rooms being merged). A room can only participate in a merge once.
//
//         r0: grid to update
//         r1: grid size x
//         r2: grid size y
//         r3: array of the starting x coordinates of each grid column
//         stack[0]: array of the starting y coordinates of each grid row
//         stack[1]: disable room merging flag
//     - name: GenerateRoomImperfections
//       address:
//         NA: 0x233ED34
//         EU: 0x233F918
//       description: |-
//         Attempt to generate room imperfections for each room in the floor layout, if enabled.
//
//         Each room has a 40% chance of having imperfections if its grid cell is flagged to allow room imperfections. Imperfections are generated by randomly growing the walls of the room inwards for a certain number of iterations, starting from the corners.
//
//         r0: grid to update
//         r1: grid size x
//         r2: grid size y
//     - name: CreateHallway
//       address:
//         NA: 0x233F120
//         EU: 0x233FD04
//       description: |-
//         Create a hallway between two points.
//
//         If the two points share no coordinates in common (meaning the line connecting them is diagonal), a "kinked" hallway is created, with the kink at a specified "middle" coordinate (in practice the grid cell boundary). For example, with a kinked horizontal hallway, there are two horizontal lines extending out from the endpoints, connected by a vertical line on the middle x coordinate.
//
//         If a hallway would intersect with an existing open tile (like an existing hallway), the hallway will only be created up to the point where it intersects with the open tile.
//
//         r0: x0
//         r1: y0
//         r2: x1
//         r3: y1
//         stack[0]: vertical flag (true for vertical hallway, false for horizontal)
//         stack[1]: middle x coordinate for kinked horizontal hallways
//         stack[2]: middle y coordinate for kinked vertical hallways
//     - name: EnsureConnectedGrid
//       address:
//         NA: 0x233F424
//         EU: 0x2340008
//       description: |-
//         Ensure the grid forms a connected graph (all valid cells are reachable) by adding hallways to unreachable grid cells.
//
//         If a grid cell cannot be connected for some reason, remove it entirely.
//
//         r0: grid to update
//         r1: grid size x
//         r2: grid size y
//         r3: array of the starting x coordinates of each grid column
//         stack[0]: array of the starting y coordinates of each grid row
//     - name: SetTerrainObstacleChecked
//       address:
//         NA: 0x233F900
//         EU: 0x23404E4
//       description: |-
//         Set the terrain of a specific tile to be an obstacle (wall or secondary terrain).
//
//         Secondary terrain (water/lava) can only be placed in the specified room. If the tile room index does not match, a wall will be placed instead.
//
//         r0: tile pointer
//         r1: use secondary terrain flag (true for water/lava, false for wall)
//         r2: room index
//     - name: FinalizeJunctions
//       address:
//         NA: 0x233F93C
//         EU: 0x2340520
//       description: |-
//         Finalizes junction tiles by setting the junction flag (bit 3 of the terrain flags) and ensuring open terrain.
//
//         Note that this implementation is slightly buggy. This function scans tiles left-to-right, top-to-bottom, and identifies junctions as any open, non-hallway tile (room_index != 0xFF) adjacent to an open, hallway tile (room_index == 0xFF). This interacts poorly with hallway anchors (room_index == 0xFE). This function sets the room index of any hallway anchors to 0xFF within the same loop, so a hallway anchor may or may not be identified as a junction depending on the orientation of connected hallways.
//
//         For example, in the following configuration, the "o" tile would be marked as a junction because the neighboring hallway tile to its left comes earlier in iteration, while the "o" tile still has the room index 0xFE, causing the algorithm to mistake it for a room tile:
//           xxxxx
//           ---ox
//           xxx|x
//           xxx|x
//         However, in the following configuration, the "o" tile would NOT be marked as a junction because it comes earlier in iteration than any of its neighboring hallway tiles, so its room index is set to 0xFF before it can be marked as a junction. This is actually the ONLY possible configuration where a hallway anchor will not be marked as a junction.
//           xxxxx
//           xo---
//           x|xxx
//           x|xxx
//
//         No params.
//     - name: GenerateKecleonShop
//       address:
//         NA: 0x233FBE8
//         EU: 0x23407CC
//       description: |-
//         Possibly generate a Kecleon shop on the floor.
//
//         A Kecleon shop will be generated with a probability determined by the Kecleon shop spawn chance parameter. A Kecleon shop will be generated in a random room that is valid, connected, has no other special features, and has dimensions of at least 5x4. Kecleon shops will occupy the entire room interior, leaving a one tile margin from the room walls.
//
//         r0: grid to update
//         r1: grid size x
//         r2: grid size y
//         r3: Kecleon shop spawn chance (percentage from 0-100)
//     - name: GenerateMonsterHouse
//       address:
//         NA: 0x233FF9C
//         EU: 0x2340B80
//       description: |-
//         Possibly generate a Monster House on the floor.
//
//         A Monster House will be generated with a probability determined by the Monster House spawn chance parameter, and only if the current floor can support one (no non-Monster-House outlaw missions or special floor types). A Monster House will be generated in a random room that is valid, connected, and is not a merged or maze room.
//
//         r0: grid to update
//         r1: grid size x
//         r2: grid size y
//         r3: Monster House spawn chance (percentage from 0-100)
//     - name: GenerateMazeRoom
//       address:
//         NA: 0x2340224
//         EU: 0x2340E08
//       description: |-
//         Possibly generate a maze room on the floor.
//
//         A maze room will be generated with a probability determined by the maze room chance parameter. A maze will be generated in a random room that is valid, connected, has odd dimensions, and has no other features.
//
//         r0: grid to update
//         r1: grid size x
//         r2: grid size y
//         r3: maze room chance (percentage from 0-100)
//     - name: GenerateMaze
//       address:
//         NA: 0x2340458
//         EU: 0x234103C
//       description: |-
//         Generate a maze room within a given grid cell.
//
//         A "maze" is generated within the room using a series of random walks to place obstacle terrain (walls or secondary terrain) in a maze-like arrangement. "Maze lines" (see GenerateMazeLine) are generated using every other tile around the room's border, as well as every other interior tile, as a starting point. This ensures that there are stripes of walkable open terrain surrounded by stripes of obstacles (the maze walls).
//
//         r0: grid cell pointer
//         r1: use secondary terrain flag (true for water/lava, false for walls)
//     - name: GenerateMazeLine
//       address:
//         NA: 0x23406D4
//         EU: 0x23412B8
//       description: |-
//         Generate a "maze line" from a given starting point, within the given bounds.
//
//         A "maze line" is a random walk starting from (x0, y0). The random walk proceeds with a stride of 2 in a random direction, laying down obstacles as it goes. The random walk terminates when it gets trapped and there are no more neighboring tiles that are open and in-bounds.
//
//         r0: x0
//         r1: y0
//         r2: xmin
//         r3: ymin
//         stack[0]: xmax
//         stack[1]: ymax
//         stack[2]: use secondary terrain flag (true for water/lava, false for walls)
//         stack[3]: room index
//     - name: SetSpawnFlag5
//       address:
//         NA: 0x234087C
//         EU: 0x2341460
//       description: |-
//         Set spawn flag 5 (0b100000 or 0x20) on all tiles in a room.
//
//         r0: grid cell
//     - name: IsNextToHallway
//       address:
//         NA: 0x23408D0
//         EU: 0x23414B4
//       description: |-
//         Checks if a tile position is either in a hallway or next to one.
//
//         r0: x
//         r1: y
//         return: bool
//     - name: ResolveInvalidSpawns
//       address:
//         NA: 0x2340974
//         EU: 0x2341558
//       description: |-
//         Resolve invalid spawn flags on tiles.
//
//         Spawn flags can be invalid due to terrain. For example, traps can't spawn on obstacles. Spawn flags can also be invalid due to multiple being set on a single tile, in which case one will take precedence. For example, stair spawns trump trap spawns.
//
//         No params.
//     - name: ConvertSecondaryTerrainToChasms
//       address:
//         NA: 0x2340A0C
//         EU: 0x23415F0
//       description: |-
//         Converts all secondary terrain tiles (water/lava) to chasms.
//
//         No params.
//     - name: EnsureImpassableTilesAreWalls
//       address:
//         NA: 0x2340A78
//         EU: 0x234165C
//       description: |-
//         Ensures all tiles with the impassable flag are walls.
//
//         No params.
//     - name: InitializeTile
//       address:
//         NA: 0x2340AD4
//         EU: 0x23416B8
//       description: |-
//         Initialize a tile struct.
//
//         r0: tile pointer
//     - name: ResetFloor
//       address:
//         NA: 0x2340B0C
//         EU: 0x23416F0
//       description: |-
//         Resets the floor in preparation for a floor generation attempt.
//
//         Resets all tiles, resets the border to be impassable, and clears entity spawns.
//
//         No params.
//     - name: PosIsOutOfBounds
//       address:
//         NA: 0x2340CAC
//         EU: 0x2341890
//       description: |-
//         Checks if a position (x, y) is out of bounds on the map: !((0 <= x <= 55) && (0 <= y <= 31)).
//
//         r0: x
//         r1: y
//         return: bool
//     - name: ShuffleSpawnPositions
//       address:
//         NA: 0x2340CE4
//         EU: 0x23418C8
//       description: |-
//         Randomly shuffle an array of spawn positions.
//
//         r0: spawn position array containing bytes {x1, y1, x2, y2, ...}
//         r1: number of (x, y) pairs in the spawn position array
//     - name: SpawnNonEnemies
//       address:
//         NA: 0x2340D4C
//         EU: 0x2341930
//       description: |-
//         Spawn all non-enemy entities, which includes stairs, items, traps, and the player.
//
//         Most entities are spawned randomly on a subset of permissible tiles.
//
//         Stairs are spawned if they don't already exist on the floor, and hidden stairs of the specified type are also spawned if configured as long as there are at least 2 floors left in the dungeon. Stairs can spawn on any tile that has open terrain, is in a room, isn't in a Kecleon shop, doesn't already have an enemy spawn, isn't a hallway junction, and isn't a special tile like a Key door.
//
//         Items are spawned both normally in rooms, as well as in walls and Monster Houses. Normal items can spawn on any tile that has open terrain, is in a room, isn't in a Kecleon shop or Monster House, isn't a hallway junction, and isn't a special tile like a Key door. Buried items can spawn on any wall tile. Monster House items can spawn on any Monster House tile that isn't in a Kecleon shop and isn't a hallway junction.
//
//         Traps are similarly spawned both normally in rooms, as well as in Monster Houses. Normal traps can spawn on any tile that has open terrain, is in a room, isn't in a Kecleon shop, doesn't already have an item or enemy spawn, and isn't a special tile like a Key door. Monster House traps follow the same conditions as Monster House items.
//
//         The player can spawn on any tile that has open terrain, is in a room, isn't in a Kecleon shop, isn't a hallway junction, doesn't already have an item, enemy, or trap spawn, and isn't a special tile like a Key door.
//
//         r0: floor properties
//         r1: empty Monster House flag. An empty Monster House is one with no items or traps, and only a small number of enemies.
//     - name: SpawnEnemies
//       address:
//         NA: 0x2341470
//         EU: 0x2342054
//       description: |-
//         Spawn all enemies, which includes normal enemies and those in Monster Houses.
//
//         Normal enemies can spawn on any tile that has open terrain, isn't in a Kecleon shop, doesn't already have another entity spawn, and isn't a special tile like a Key door.
//
//         Monster House enemies can spawn on any Monster House tile that isn't in a Kecleon shop, isn't where the player spawns, and isn't a special tile like a Key door.
//
//         r0: floor properties
//         r1: empty Monster House flag. An empty Monster House is one with no items or traps, and only a small number of enemies.
//     - name: SetSecondaryTerrainOnWall
//       address:
//         NA: 0x234176C
//         EU: 0x2342350
//       description: |-
//         Set a specific tile to have secondary terrain (water/lava), but only if it's a passable wall.
//
//         r0: tile pointer
//     - name: GenerateSecondaryTerrainFormations
//       address:
//         NA: 0x23417AC
//         EU: 0x2342390
//       description: |-
//         Generate secondary terrain (water/lava) formations.
//
//         This includes "rivers" that flow from top-to-bottom (or bottom-to-top), as well as "lakes" both standalone and after rivers. Water/lava formations will never cut through rooms, but they can pass through rooms to the opposite side.
//
//         Rivers are generated by a top-down or bottom-up random walk that ends when existing secondary terrain is reached or the walk goes out of bounds. Some rivers also end prematurely in a lake. Lakes are a large collection of secondary terrain generated around a central point.
//
//         r0: bit index to test in the floor properties room flag bitvector (formations are only generated if the bit is set)
//         r1: floor properties
//     - name: StairsAlwaysReachable
//       address:
//         NA: 0x2341E6C
//         EU: 0x2342A50
//       description: |-
//         Checks that the stairs are reachable from every walkable tile on the floor.
//
//         This runs a graph traversal algorithm that is very similar to breadth-first search (the order in which nodes are visited is slightly different), starting from the stairs. If any tile is walkable but wasn't reached by the traversal algorithm, then the stairs must not be reachable from that tile.
//
//         r0: x coordinate of the stairs
//         r1: y coordinate of the stairs
//         r2: flag to always return true, but set a special bit on all walkable tiles that aren't reachable from the stairs
//         return: bool
//     - name: ConvertWallsToChasms
//       address:
//         NA: 0x2342548
//         EU: 0x234312C
//       description: |-
//         Converts all wall tiles to chasms.
//
//         No params.
//     - name: ResetInnerBoundaryTileRows
//       address:
//         NA: 0x2342B7C
//         EU: 0x2343760
//       description: |-
//         Reset the inner boundary tile rows (y == 1 and y == 30) to their initial state of all wall tiles, with impassable walls at the edges (x == 0 and x == 55).
//
//         No params.
//     - name: SpawnStairs
//       address:
//         NA: 0x2342C8C
//         EU: 0x2343870
//       description: |-
//         Spawn stairs at the given location.
//
//         If the hidden stairs flag is set, hidden stairs will be spawned instead of normal stairs.
//
//         If spawning normal stairs and the current floor is a rescue floor, the room containing the stairs will be converted into a Monster House.
//
//         r0: position (two-byte array for {x, y})
//         r1: dungeon generation info pointer (a field on the dungeon struct)
//         r2: hidden stairs flag
//     - name: LoadFixedRoomData
//       address:
//         NA: 0x2343D90
//         EU: 0x2344974
//       description: |-
//         Loads fixed room data from BALANCE/fixed.bin into the buffer pointed to by FIXED_ROOM_DATA_PTR.
//
//         No params.
//     - name: IsHiddenStairsFloor
//       address:
//         NA: 0x234450C
//         EU: 0x23450F0
//       description: |-
//         Checks if the current floor is either the Secret Bazaar or a Secret Room.
//
//         return: bool
//     - name: HasHeldItem
//       address:
//         NA: 0x23467E4
//         EU: 0x23473D0
//       description: |-
//         Checks if a monster has a certain held item.
//
//         r0: entity pointer
//         r1: item ID
//         return: bool
//     - name: IsCurrentMissionType
//       address:
//         NA: 0x234921C
//         EU: 0x2349E1C
//       description: |-
//         Checks if the current floor is an active mission destination of a given type (and any subtype).
//
//         r0: mission type
//         return: bool
//     - name: IsCurrentMissionTypeExact
//       address:
//         NA: 0x2349250
//         EU: 0x2349E50
//       description: |-
//         Checks if the current floor is an active mission destination of a given type and subtype.
//
//         r0: mission type
//         r1: mission subtype
//         return: bool
//     - name: IsOutlawMonsterHouseFloor
//       address:
//         NA: 0x234928C
//         EU: 0x2349E8C
//       description: |-
//         Checks if the current floor is a mission destination for a Monster House outlaw mission.
//
//         return: bool
//     - name: IsGoldenChamber
//       address:
//         NA: 0x23492B0
//         EU: 0x2349EB0
//       description: |-
//         Checks if the current floor is a Golden Chamber floor.
//
//         return: bool
//     - name: IsLegendaryChallengeFloor
//       address:
//         NA: 0x23492D4
//         EU: 0x2349ED4
//       description: |-
//         Checks if the current floor is a boss floor for a Legendary Challenge Letter mission.
//
//         return: bool
//     - name: IsJirachiChallengeFloor
//       address:
//         NA: 0x2349314
//         EU: 0x2349F14
//       description: |-
//         Checks if the current floor is the boss floor in Star Cave Pit for Jirachi's Challenge Letter mission.
//
//         return: bool
//     - name: IsDestinationFloorWithMonster
//       address:
//         NA: 0x234934C
//         EU: 0x2349F4C
//       description: |-
//         Checks if the current floor is a mission destination floor with a special monster.
//
//         See FloorHasMissionMonster for details.
//
//         return: bool
//     - name: MissionTargetEnemyIsDefeated
//       address:
//         NA: 0x2349470
//         EU: 0x234A070
//       description: |-
//         Checks if the target enemy of the mission on the current floor has been defeated.
//
//         return: bool
//     - name: SetMissionTargetEnemyDefeated
//       address:
//         NA: 0x2349490
//         EU: 0x234A090
//       description: |-
//         Set the flag for whether or not the target enemy of the current mission has been defeated.
//
//         r0: new flag value
//     - name: IsDestinationFloorWithFixedRoom
//       address:
//         NA: 0x23494A4
//         EU: 0x234A0A4
//       description: |-
//         Checks if the current floor is a mission destination floor with a fixed room.
//
//         The entire floor can be a fixed room layout, or it can just contain a Sealed Chamber.
//
//         return: bool
//     - name: GetItemToRetrieve
//       address:
//         NA: 0x23494CC
//         EU: 0x234A0CC
//       description: |-
//         Get the ID of the item that needs to be retrieve on the current floor for a mission, if one exists.
//
//         return: item ID
//     - name: GetItemToDeliver
//       address:
//         NA: 0x23494F0
//         EU: 0x234A0F0
//       description: |-
//         Get the ID of the item that needs to be delivered to a mission client on the current floor, if one exists.
//
//         return: item ID
//     - name: GetSpecialTargetItem
//       address:
//         NA: 0x234951C
//         EU: 0x234A11C
//       description: |-
//         Get the ID of the special target item for a Sealed Chamber or Treasure Memo mission on the current floor.
//
//         return: item ID
//     - name: IsDestinationFloorWithItem
//       address:
//         NA: 0x2349564
//         EU: 0x234A164
//       description: |-
//         Checks if the current floor is a mission destination floor with a special item.
//
//         This excludes missions involving taking an item from an outlaw.
//
//         return: bool
//     - name: IsDestinationFloorWithHiddenOutlaw
//       address:
//         NA: 0x23495C4
//         EU: 0x234A1C4
//       description: |-
//         Checks if the current floor is a mission destination floor with a "hidden outlaw" that behaves like a normal enemy.
//
//         return: bool
//     - name: IsDestinationFloorWithFleeingOutlaw
//       address:
//         NA: 0x23495E8
//         EU: 0x234A1E8
//       description: |-
//         Checks if the current floor is a mission destination floor with a "fleeing outlaw" that runs away.
//
//         return: bool
//     - name: GetMissionTargetEnemy
//       address:
//         NA: 0x2349620
//         EU: 0x234A220
//       description: |-
//         Get the monster ID of the target enemy to be defeated on the current floor for a mission, if one exists.
//
//         return: monster ID
//     - name: GetMissionEnemyMinionGroup
//       address:
//         NA: 0x2349638
//         EU: 0x234A238
//       description: |-
//         Get the monster ID of the specified minion group on the current floor for a mission, if it exists.
//
//         Note that a single minion group can correspond to multiple actual minions of the same species. There can be up to 2 minion groups.
//
//         r0: minion group index (0-indexed)
//         return: monster ID
//     - name: FloorHasMissionMonster
//       address:
//         NA: 0x2349748
//         EU: 0x234A348
//       description: |-
//         Checks if a given floor is a mission destination with a special monster, either a target to rescue or an enemy to defeat.
//
//         Mission types with a monster on the destination floor:
//         - Rescue client
//         - Rescue target
//         - Escort to target
//         - Deliver item
//         - Search for target
//         - Take item from outlaw
//         - Arrest outlaw
//         - Challenge Request
//
//         r0: mission destination info pointer
//         return: bool
//     - name: RunDungeonMode
//       address:
//         NA: 0x234BF28
//         EU: 0x234CB28
//       description: |-
//         This appears to be the top-level function for running dungeon mode.
//
//         It gets called by some code in overlay 10 right after doing the dungeon fade transition, and once it exits, the dungeon results are processed.
//
//         This function is presumably in charge of allocating the dungeon struct, setting it up, launching the dungeon engine, etc.
//     - name: SetBothScreensWindowColorToDefault
//       address:
//         NA: 0x234CF60
//         EU: 0x234DB60
//       description: |-
//         This changes the palettes of windows in both screens to an appropiate value depending on the playthrough
//         If you're in a special episode, they turn green , otherwise, they turn blue or pink depending on your character's sex
//
//         No params
//   data:
//     - name: DUNGEON_STRUCT_SIZE
//       address:
//         NA:
//           - 0x22DEA78
//           - 0x22DEAAC
//       length:
//         NA: 0x4
//       description: Size of the dungeon struct (0x2CB14)
//     - name: OFFSET_OF_DUNGEON_FLOOR_PROPERTIES
//       address:
//         NA:
//           - 0x22E79F8
//           - 0x233AE68
//       length:
//         NA: 0x4
//       description: Offset of the floor properties field in the dungeon struct
//     - name: DUNGEON_PRNG_LCG_MULTIPLIER
//       address:
//         NA:
//           - 0x22EA9C8
//           - 0x22EAA8C
//       length:
//         NA: 0x4
//       description: "The multiplier shared by all of the dungeon PRNG's LCGs, 1566083941 (0x5D588B65)."
//     - name: DUNGEON_PRNG_LCG_INCREMENT_SECONDARY
//       address:
//         NA: 0x22EAA94
//       length:
//         NA: 0x4
//       description: "The increment for the dungeon PRNG's secondary LCGs, 2531011 (0x269EC3). This happens to be the same increment that the Microsoft Visual C++ runtime library uses in its implementation of the rand() function."
//     - name: EXPERIENCE_POINT_GAIN_CAP
//       address:
//         NA: 0x23026C8
//       length:
//         NA: 0x4
//       description: A cap on the experience that can be given to a monster in one call to AddExpSpecial
//     - name: JUDGMENT_MOVE_ID
//       address:
//         NA: 0x230C458
//       length:
//         NA: 0x4
//       description: |-
//         Move ID for Judgment
//
//         type: enum move_id
//     - name: REGULAR_ATTACK_MOVE_ID
//       address:
//         NA: 0x230C45C
//       length:
//         NA: 0x4
//       description: |-
//         Move ID for the regular attack
//
//         type: enum move_id
//     - name: DEOXYS_ATTACK_ID
//       address:
//         NA: 0x230C460
//       length:
//         NA: 0x4
//       description: |-
//         Monster ID for Deoxys in Attack Forme
//
//         type: enum monster_id
//     - name: DEOXYS_SPEED_ID
//       address:
//         NA: 0x230C464
//       length:
//         NA: 0x4
//       description: |-
//         Monster ID for Deoxys in Speed Forme
//
//         type: enum monster_id
//     - name: GIRATINA_ALTERED_ID
//       address:
//         NA: 0x230C468
//       length:
//         NA: 0x4
//       description: |-
//         Monster ID for Giratina in Altered Forme
//
//         type: enum monster_id
//     - name: PUNISHMENT_MOVE_ID
//       address:
//         NA: 0x230C46C
//       length:
//         NA: 0x4
//       description: |-
//         Move ID for Punishment
//
//         type: enum move_id
//     - name: OFFENSE_STAT_MAX
//       address:
//         NA: 0x230C49C
//       length:
//         NA: 0x4
//       description: "Cap on an attacker's modified offense (attack or special attack) stat after boosts. Used during damage calculation."
//     - name: PROJECTILE_MOVE_ID
//       address:
//         NA:
//           - 0x230D07C
//           - 0x231C700
//       length:
//         NA: 0x4
//       description: |-
//         The move ID of the special "projectile" move
//
//         type: enum move_id
//     - name: BELLY_LOST_PER_TURN
//       address:
//         NA: 0x2310A70
//         EU: 0x23114D0
//       length:
//         NA: 0x4
//         EU: 0x4
//       description: |-
//         The base value by which belly is decreased every turn.
//
//         Its raw value is 0x199A, which encodes a Q16.16 binary fixed-point float with value (0x199A * 2^-16), and is the closest approximation to 0.1 representable in this number format.
//     - name: MOVE_TARGET_AND_RANGE_SPECIAL_USER_HEALING
//       address:
//         NA: 0x231AD34
//       length:
//         NA: 0x4
//       description: |-
//         The move target and range code for special healing moves that target just the user.
//
//         type: struct move_target_and_range (+ padding)
//     - name: PLAIN_SEED_VALUE
//       address:
//         NA: 0x231C748
//       length:
//         NA: 0x4
//       description: Some value related to the Plain Seed.
//     - name: MAX_ELIXIR_PP_RESTORATION
//       address:
//         NA: 0x231C74C
//       length:
//         NA: 0x4
//       description: The amount of PP restored per move by ingesting a Max Elixir.
//     - name: SLIP_SEED_VALUE
//       address:
//         NA: 0x231CBAC
//       length:
//         NA: 0x4
//       description: Some value related to the Slip Seed.
//     - name: FLOOR_GENERATION_STATUS_PTR
//       address:
//         NA:
//           - 0x233AE6C
//           - 0x233AF08
//           - 0x233B18C
//           - 0x233B618
//           - 0x233BA78
//           - 0x233BBD8
//           - 0x233BD70
//           - 0x233BF2C
//           - 0x233C30C
//           - 0x233C76C
//           - 0x233CF80
//           - 0x233D100
//           - 0x233D310
//           - 0x233D670
//           - 0x233E058
//           - 0x233FF90
//           - 0x234021C
//           - 0x23406D0
//           - 0x234145C
//           - 0x2341764
//           - 0x2342178
//           - 0x2342510
//           - 0x23427E0
//           - 0x2342B74
//           - 0x2342C64
//           - 0x2342D98
//           - 0x2342F28
//       length:
//         NA: 0x4
//       description: |-
//         Pointer to the global FLOOR_GENERATION_STATUS
//
//         type: struct floor_generation_status*
//     - name: OFFSET_OF_DUNGEON_N_NORMAL_ITEM_SPAWNS
//       address:
//         NA:
//           - 0x233AE74
//           - 0x2341464
//       length:
//         NA: 0x4
//       description: Offset of the (number of base items + 1) field on the dungeon struct
//     - name: DUNGEON_GRID_COLUMN_BYTES
//       address:
//         NA:
//           - 0x233B614
//           - 0x233BA74
//           - 0x233BD6C
//           - 0x233BF28
//           - 0x233C308
//           - 0x233C770
//           - 0x233C9E4
//           - 0x233CF78
//           - 0x233D0FC
//           - 0x233D314
//           - 0x233D66C
//           - 0x233E054
//           - 0x233E438
//           - 0x233ED30
//           - 0x233F114
//           - 0x233F8FC
//           - 0x233FF94
//           - 0x2340220
//           - 0x2340454
//           - 0x23424CC
//       length:
//         NA: 0x4
//       description: "The number of bytes in one column of the dungeon grid cell array, 450, which corresponds to a column of 15 grid cells."
//     - name: DEFAULT_MAX_POSITION
//       address:
//         NA: 0x233FF98
//       length:
//         NA: 0x4
//       description: A large number (9999) to use as a default position for keeping track of min/max position values
//     - name: OFFSET_OF_DUNGEON_GUARANTEED_ITEM_ID
//       address:
//         NA:
//           - 0x2341460
//           - 0x2344E80
//       length:
//         NA: 0x4
//       description: Offset of the guaranteed item ID field in the dungeon struct
//     - name: FIXED_ROOM_TILE_SPAWN_TABLE
//       address:
//         NA: 0x234FDD0
//         EU: 0x23509DC
//       length:
//         NA: 0x2C
//         EU: 0x2C
//       description: |-
//         Table of tiles that can spawn in fixed rooms, pointed into by the FIXED_ROOM_TILE_SPAWN_TABLE.
//
//         This is an array of 11 4-byte entries containing info about one tile each. Info includes the trap ID if a trap, room ID, and flags.
//
//         type: struct fixed_room_tile_spawn_entry[11]
//     - name: FIXED_ROOM_REVISIT_OVERRIDES
//       address:
//         NA: 0x234FE14
//         EU: 0x2350A20
//       length:
//         NA: 0x100
//         EU: 0x100
//       description: |-
//         Table of fixed room IDs, which if nonzero, overrides the normal fixed room ID for a floor (which is used to index the table) if the dungeon has already been cleared previously.
//
//         Overrides are used to substitute different fixed room data for things like revisits to story dungeons.
//
//         type: struct fixed_room_id_8[256]
//     - name: FIXED_ROOM_MONSTER_SPAWN_TABLE
//       address:
//         NA: 0x234FF14
//         EU: 0x2350B20
//       length:
//         NA: 0x1E0
//         EU: 0x1E0
//       description: |-
//         Table of monsters that can spawn in fixed rooms, pointed into by the FIXED_ROOM_ENTITY_SPAWN_TABLE.
//
//         This is an array of 120 4-byte entries containing info about one monster each. Info includes the monster ID, stats, and behavior type.
//
//         type: struct fixed_room_monster_spawn_entry[120]
//     - name: FIXED_ROOM_ITEM_SPAWN_TABLE
//       address:
//         NA: 0x23500F4
//         EU: 0x2350D00
//       length:
//         NA: 0x1F8
//         EU: 0x1F8
//       description: |-
//         Table of items that can spawn in fixed rooms, pointed into by the FIXED_ROOM_ENTITY_SPAWN_TABLE.
//
//         This is an array of 63 8-byte entries containing one item ID each.
//
//         type: struct fixed_room_item_spawn_entry[63]
//     - name: FIXED_ROOM_ENTITY_SPAWN_TABLE
//       address:
//         NA: 0x23502EC
//         EU: 0x2350EF8
//       length:
//         NA: 0xC9C
//         EU: 0xC9C
//       description: |-
//         Table of entities (items, monsters, tiles) that can spawn in fixed rooms, which is indexed into by the main data structure for each fixed room.
//
//         This is an array of 269 entries. Each entry contains 3 pointers (one into FIXED_ROOM_ITEM_SPAWN_TABLE, one into FIXED_ROOM_MONSTER_SPAWN_TABLE, and one into FIXED_ROOM_TILE_SPAWN_TABLE), and represents the entities that can spawn on one specific tile in a fixed room.
//
//         type: struct fixed_room_entity_spawn_entry[269]
//     - name: DIRECTIONS_XY
//       address:
//         NA: 0x235171C
//       length:
//         NA: 0x20
//       description: |-
//         An array mapping each direction index to its x and y displacements.
//
//         Directions start with 0=down and proceed counterclockwise (see enum direction_id). Displacements for x and y are interleaved and encoded as 2-byte signed integers. For example, the first two integers are [0, 1], which correspond to the x and y displacements for the "down" direction (positive y means down).
//     - name: BELLY_DRAIN_IN_WALLS_INT
//       address:
//         NA: 0x2352768
//         EU: 0x2353374
//       length:
//         NA: 0x2
//         EU: 0x2
//       description: The additional amount by which belly is decreased every turn when inside walls (integer part)
//     - name: BELLY_DRAIN_IN_WALLS_THOUSANDTHS
//       address:
//         NA: 0x235276A
//         EU: 0x2353376
//       length:
//         NA: 0x2
//         EU: 0x2
//       description: The additional amount by which belly is decreased every turn when inside walls (fractional thousandths)
//     - name: SPATK_STAT_IDX
//       address:
//         NA: 0x2352AE8
//       length:
//         NA: 0x4
//       description: "The index (1) of the special attack entry in internal stat structs, such as the stat modifier array for a monster."
//     - name: ATK_STAT_IDX
//       address:
//         NA: 0x2352AEC
//       length:
//         NA: 0x4
//       description: "The index (0) of the attack entry in internal stat structs, such as the stat modifier array for a monster."
//     - name: CORNER_CARDINAL_NEIGHBOR_IS_OPEN
//       address:
//         NA: 0x2353010
//       length:
//         NA: 0x20
//       description: |-
//         An array mapping each (corner index, neighbor direction index) to whether or not that neighbor is expected to be open floor.
//
//         Corners start with 0=top-left and proceed clockwise. Directions are enumerated as with DIRECTIONS_XY. The array is indexed by i=(corner_index * N_DIRECTIONS + direction). An element of 1 (0) means that starting from the specified corner of a room, moving in the specified direction should lead to an open floor tile (non-open terrain like a wall).
//
//         Note that this array is only used for the cardinal directions. The elements at odd indexes are unused and unconditionally set to 0.
//
//         This array is used by the dungeon generation algorithm when generating room imperfections. See GenerateRoomImperfections.
//     - name: DUNGEON_PTR
//       address:
//         NA: 0x2353538
//         EU: 0x2354138
//       length:
//         NA: 0x4
//         EU: 0x4
//       description: |-
//         [Runtime] Pointer to the dungeon struct in dungeon mode.
//
//         This is a "working copy" of DUNGEON_PTR_MASTER. The main dungeon engine uses this pointer (or rather pointers to this pointer) when actually running dungeon mode.
//
//         type: struct dungeon*
//     - name: DUNGEON_PTR_MASTER
//       address:
//         NA: 0x235353C
//       length:
//         NA: 0x4
//       description: |-
//         [Runtime] Pointer to the dungeon struct in dungeon mode.
//
//         This is a "master copy" of the dungeon pointer. The game uses this pointer when doing low-level memory work (allocation, freeing, zeroing). The normal DUNGEON_PTR is used for most other dungeon mode work.
//
//         type: struct dungeon*
//     - name: DUNGEON_PRNG_STATE
//       address:
//         NA: 0x2353570
//       length:
//         NA: 0x14
//       description: |-
//         [Runtime] The global PRNG state for dungeon mode, not including the current values in the secondary sequences.
//
//         This struct holds state for the primary LCG, as well as the current configuration controlling which LCG to use when generating random numbers. See DungeonRand16Bit for more information on how the dungeon PRNG works.
//
//         type: struct prng_state
//     - name: DUNGEON_PRNG_STATE_SECONDARY_VALUES
//       address:
//         NA: 0x2353584
//       length:
//         NA: 0x14
//       description: |-
//         [Runtime] An array of 5 integers corresponding to the last value generated for each secondary LCG sequence.
//
//         Based on the assembly, this appears to be its own global array, separate from DUNGEON_PRNG_STATE.
//     - name: DEFAULT_TILE
//       address:
//         NA: 0x2353724
//       length:
//         NA: 0x14
//       description: |-
//         The default tile struct.
//
//         This is just a struct full of zeroes, but is used as a fallback in various places where a "default" tile is needed, such as when a grid index is out of range.
//
//         type: struct tile
//     - name: FIXED_ROOM_DATA_PTR
//       address:
//         NA: 0x2353794
//       length:
//         NA: 0x4
//       description: "[Runtime] Pointer to decoded fixed room data loaded from the BALANCE/fixed.bin file."

/// Actual effects impl block.
/// TODO: NOT NEARLY FINISHED
impl DungeonEffectsEmitter {
    pub fn try_inflict_burn(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        special_effect: bool,
        log_failure: bool,
        check_only: bool
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictBurnStatus(
            attacker as *mut _, defender as *mut _,
            special_effect as ffi::bool_, log_failure as ffi::bool_,
            check_only as ffi::bool_
        ) > 0 }
    }

    pub fn try_inflict_bad_poison(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        log_failure: bool,
        check_only: bool
    ) -> bool {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::TryInflictBadlyPoisonedStatus(
            attacker as *mut _, defender as *mut _,
            log_failure as ffi::bool_, check_only as ffi::bool_
        ) > 0 }
    }

    pub fn deal_damage(
        &self,
        attacker: &mut DungeonEntity,
        defender: &mut DungeonEntity,
        used_move: &mut DungeonMove,
        damage_multiplier: I24F8,
        item_id: Option<item_catalog::Type>
    ) -> i32 {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::DealDamage(
            attacker as *mut _, defender as *mut _,
            used_move as *mut _, damage_multiplier.to_bits() as c_int,
            item_id.unwrap_or(item_catalog::ITEM_NOTHING)
        ) }
    }
}

/// Builder for creating dungeon message log messages.
/// By default message will be shown 'quiet', meaning there will be no popup
/// shown when the message is logged. You can force a popup to be shown with `Self::popup`,
/// but please also note that with some configurations, a popup will always be displayed, even
/// if `Self::popup` is not called. See the implementation for more details.
pub struct LogMessageBuilder<'a> {
    _lease: OverlayLoadLease<29>,
    popup: bool,
    check_user: bool,
    target_check_fainted: Option<&'a DungeonEntity>
}

impl<'a> CreatableWithLease<29> for LogMessageBuilder<'a> {
    fn _create(lease: OverlayLoadLease<29>) -> Self {
        Self {
            _lease: lease,
            popup: false,
            check_user: false,
            target_check_fainted: None
        }
    }

    fn lease(&self) -> &OverlayLoadLease<29> {
        &self._lease
    }
}

impl<'a> LogMessageBuilder<'a> {
    /// Show a message popup when the message is displayed.
    pub fn popup(&mut self) -> &mut Self {
        self.popup = true;
        self
    }

    /// Do not show the message if the user is fainted.
    ///
    /// # Note
    /// `target_check_fainted` will take precedence over this, both can not be active
    /// at the same time.
    pub fn check_user_fainted(&'a mut self) -> &'a mut Self {
        self.check_user = true;
        self
    }

    // Do not show the message if the target is fainted and an unknown check
    // regarding the user passes.
    pub fn target_check_fainted(&'a mut self, user: &'a DungeonEntity, target: &'a DungeonEntity) -> &'a mut Self {
        self.target_check_fainted = Some(target);
        self
    }

    /// Replaces instances of a given placeholder tag by the string representation of the given entity.
    /// Concretely this means that any occurrences of `[string:<string_id>]` will be replaced by the
    /// name of the given entity.
    /// Example: If use pass `string_id` with 1, it will replace all occurrences of `[string:1]`.
    ///
    /// # Note
    /// As a performance optimization this will immediately reserve that string with the game when
    /// called. This can have weird effects if you expect to show the message built by this builder
    /// at a later time.
    pub fn string(&mut self, string_id: u16, entity: &DungeonEntity) -> &mut Self {
        // SAFETY: We have a lease on the overlay existing.
        unsafe { ffi::SubstitutePlaceholderStringTags(
            string_id as ctypes::c_int, force_mut_ptr!(entity), 0
        ) }
        self
    }

    /// Writes a log entry using the message with the given message ID.
    pub fn log_msg(&mut self, user: &DungeonEntity, message_id: i32) {
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            match (self.popup, self.check_user, self.target_check_fainted) {
                (false, false, None) => ffi::LogMessageByIdQuiet(force_mut_ptr!(user), message_id),
                (_,     true,  None) => ffi::LogMessageByIdWithPopupCheckUser(force_mut_ptr!(user), message_id),
                (false, _,     Some(target)) => ffi::LogMessageByIdQuietCheckUserTarget(force_mut_ptr!(user), force_mut_ptr!(target), message_id,),
                (true,  false, None) => ffi::LogMessageByIdWithPopup(force_mut_ptr!(user), message_id),
                (true,  _,     Some(target)) => ffi::LogMessageByIdWithPopupCheckUserTarget(force_mut_ptr!(user), force_mut_ptr!(target), message_id,),
            }
        }
    }

    pub fn log_str<S: AsRef<str> + Debug>(&mut self, user: &DungeonEntity, message: S) {
        self.log_cstr(user, str_to_cstring(message))
    }

    pub fn log_cstr<S: AsRef<CStr>>(&mut self, user: &DungeonEntity, message: S) {
        let message = message.as_ref().as_ptr() as *const c_char;
        // SAFETY: We have a lease on the overlay existing.
        unsafe {
            match (self.popup, self.check_user, self.target_check_fainted) {
                (false, false, None) => ffi::LogMessageQuiet(force_mut_ptr!(user), message),
                (_,     true,  None) => ffi::LogMessageWithPopupCheckUser(force_mut_ptr!(user), message),
                (true,  false, None) => ffi::LogMessageWithPopup(force_mut_ptr!(user), message),
                (_,     _,     Some(target)) => ffi::LogMessageWithPopupCheckUserTarget(force_mut_ptr!(user), force_mut_ptr!(target), message,),
            }
        }
    }
}



// Misc dungeon functions.

pub fn entity_is_valid(entity: &mut DungeonEntity, _ov29: &OverlayLoadLease<29>) -> bool {
    // SAFETY: The lease passed into the function promises us that the overlay is loaded.
    unsafe { ffi::EntityIsValid(entity as *mut DungeonEntity) > 0 }
}

pub fn dungeon_rand_100(_ov29: &OverlayLoadLease<29>) -> u32 {
    // SAFETY: The lease passed into the function promises us that the overlay is loaded.
    unsafe { ffi::DungeonRand100() }
}
