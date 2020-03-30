use std::marker::PhantomData;

use crate::base::*;

pub enum Sum < P, Q > {
  Inl ( P ),
  Inr ( Q )
}

pub trait ProtocolField < P >
where
  P : Protocol
{
  type Value : Send;
}

pub trait ConvergeSum < T, P, R >
where
  P : Protocol,
  T : ProtocolField < P >
{
  fn match_proc
    ( self,
      val : T :: Value ) ->
      R
  ;
}

pub trait ProtocolSum < T >
{
  type ValueSum : Send;
}

pub trait ProtocolPrism
  < S, P, T >
where
  P : Protocol,
  S : ProtocolSum < T >,
  T : ProtocolField < P >
{
  fn intro_sum
    ( val : T :: Value )
    -> S :: ValueSum;

  fn elim_sum
    ( val_sum : S :: ValueSum )
    ->
      Option <
        T :: Value
      >;
}

pub trait MergeProtocolSum < T1, T2 >
  : ProtocolSum < T1 >
  + ProtocolSum < T2 >
  + ProtocolSum <
      ToMerge < T1, T2 >
    >
{
  fn merge_sum (
    sum1 :
      < Self as
        ProtocolSum < T1 >
      > :: ValueSum,
    sum2 :
      < Self as
        ProtocolSum < T2 >
      > :: ValueSum
  ) ->
    Option <
      < Self as
        ProtocolSum <
          ToMerge < T1, T2 >
        >
      > :: ValueSum
    >
  ;
}

pub struct ToValue { }

pub struct ToUnit { }

pub struct ToSession < I >
{
  i : PhantomData < I >
}

pub struct ToMerge < T1, T2 >
{
  t1 : PhantomData < T1 >,
  t2 : PhantomData < T2 >
}

impl
  < T, P >
  ProtocolSum < T >
  for P
where
  P : Protocol,
  T : ProtocolField < P >,
{
  type ValueSum = T :: Value;
}

impl
  < P, T >
  ProtocolPrism
  < P, P, T >
  for Z
where
  P : Protocol,
  T : ProtocolField < P >
{
  fn intro_sum
    ( val : T :: Value )
    -> T :: Value
  {
    val
  }

  fn elim_sum
    ( val : T :: Value )
    ->
      Option < T :: Value >
  {
    Some ( val )
  }
}

impl
  < P, R, T >
  ProtocolPrism <
    (P, R),
    P,
    T
  >
  for Z
where
  P : Protocol,
  R : ProtocolSum < T >,
  T : ProtocolField < P >
{
  fn intro_sum
    ( val : T :: Value )
    ->
      Sum <
        T :: Value,
        R :: ValueSum
      >
  {
    Sum::Inl ( val )
  }

  fn elim_sum
    ( val_sum :
      Sum <
        T :: Value,
        R :: ValueSum
      >
    ) ->
      Option <
        T :: Value
      >
  {
    match val_sum {
      Sum::Inl ( val ) => {
        Option::Some ( val )
      },
      Sum::Inr ( _ ) => {
        Option::None
      }
    }
  }
}

impl
  < N, Q, P, R, T >
  ProtocolPrism <
    (Q, R),
    P,
    T
  >
  for S < N >
where
  N : Nat,
  P : Protocol,
  Q : Protocol,
  R : ProtocolSum < T >,
  T : ProtocolField < P >,
  T : ProtocolField < Q >,
  N : ProtocolPrism < R, P, T >
{

  fn intro_sum
    ( val :
        < T as
          ProtocolField < P >
        > :: Value )
    ->
      Sum <
        < T as
          ProtocolField < Q >
        > :: Value,
        R :: ValueSum
      >
  {
    Sum::Inr (
      N :: intro_sum ( val )
    )
  }

  fn elim_sum
    ( val_sum :
      Sum <
        < T as
          ProtocolField < Q >
        > :: Value,
        R :: ValueSum
      >
    ) ->
      Option <
        < T as
          ProtocolField < P >
        > :: Value
      >
  {
    match val_sum {
      Sum::Inl ( _ ) => {
        Option::None
      },
      Sum::Inr ( val_sum2 ) => {
        N :: elim_sum ( val_sum2 )
      }
    }
  }
}


impl
  < T, P, R >
  ProtocolSum < T >
  for (P, R)
where
  P : Protocol,
  T : ProtocolField < P >,
  R : ProtocolSum < T >
{
  type ValueSum =
    Sum <
      T :: Value,
      R :: ValueSum
    >;
}

impl < P >
  ProtocolField < P >
  for ToValue
where
  P : Protocol
{
  type Value = P :: Payload;
}

impl < P >
  ProtocolField < P >
  for ToUnit
where
  P : Protocol
{
  type Value = ();
}

impl < I, P >
  ProtocolField < P >
  for ToSession < I >
where
  P : Protocol,
  I : Context
{
  type Value =
    PartialSession < I, P >;
}

impl
  < P, T1, T2 >
  ProtocolField < P >
  for ToMerge < T1, T2 >
where
  P : Protocol,
  T1 : ProtocolField < P >,
  T2 : ProtocolField < P >,
{
  type Value =
    ( T1 :: Value,
      T2 :: Value
    );
}
