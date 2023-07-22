-module(simpler_nif).

-export([ get_params_erl/1
        , get_params/1
        ]).

-include("cargo.hrl").
-on_load(init/0).
-define(NOT_LOADED, not_loaded(?LINE)).

%%%===================================================================
%%% API
%%%===================================================================

get_params_erl(Ns) ->
    {Qs, L} = get_quartiles_length(Ns),
    S = lists:sum(Ns),
    M = S / L,
    #{sum => S,
      mean => M,
      quartiles => Qs
     }.

get_params(_Ns) ->
    ?NOT_LOADED.

%%%===================================================================
%%% Private
%%%===================================================================

get_quartiles_length(Ns) ->
    L = length(Ns),
    Ss = lists:sort(Ns),
    X = L div 2,
    {Q1, Q3} =
        case L rem 2 of
            0 -> {median_from_sorted(lists:sublist(Ss, X), X),
                  median_from_sorted(lists:sublist(Ss, X + 1, X), X)};
            1 -> {median_from_sorted(lists:sublist(Ss, X + 1), X + 1),
                  median_from_sorted(lists:sublist(Ss, X + 1, X + 1), X + 1)}
        end,
    Median = median_from_sorted(Ss, L),
    Qs = [Q1, Median, Q3],
    {Qs, L}.

median_from_sorted(Ns, L) ->
    A = lists:nth((L div 2), Ns),
    B = lists:nth((L div 2) + 1, Ns),
    case L rem 2 of
        1 -> B;
        0 -> (A + B) / 2
    end.

%%%===================================================================
%%% NIF
%%%===================================================================

init() ->
    ?load_nif_from_crate(simpler_nif, 0).

not_loaded(Line) ->
    erlang:nif_error({not_loaded, [{module, ?MODULE}, {line, Line}]}).

%%%===================================================================
%%% Tests
%%%===================================================================

-ifdef(TEST).
-include_lib("eunit/include/eunit.hrl").

vec_in_length_even_test() ->
    ?assertEqual(#{mean => 4.5,
                   quartiles => [2.5,4.5,6.5],
                   sum => 36},
                 get_params([8,6,4,5,7,1,3,2])).

vec_in_length_odd_test() ->
    ?assertEqual(#{mean => 5.0,
                   quartiles => [3.0,5.0,7.0],
                   sum => 45},
                 get_params([9,8,6,4,5,7,1,3,2])).

-endif.
